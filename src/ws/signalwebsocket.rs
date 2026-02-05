use async_trait::async_trait;
use eyre::{eyre, Result};
use futures_channel::mpsc;
use http::StatusCode;
use prost::Message;
use rocket::serde::json::serde_json::json;
use std::{
    fmt::{Display, Formatter},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::time;
use tokio_tungstenite::tungstenite;

use super::tls;
use super::websocket_connection::{self, WebSocketConnection};
use super::{
    proto_signalservice::Envelope,
    proto_websocketresources::{
        web_socket_message::Type, WebSocketMessage, WebSocketRequestMessage,
        WebSocketResponseMessage,
    },
};
use crate::{config, utils::post_allowed::post_allowed};

/// Time between 2 regular push notifications
///
/// We don't need to send a push notif per message, as
/// the client stay connected for more than 10secs on push
const PUSH_TIMEOUT: Duration = Duration::from_secs(1);
/// Time between last push notification and a delivery check
///
/// A delivery check is a push notif used to control the HTTP reponse of the push endpoint,
/// to check if we have the good one.
/// The delivery check is useful in case the user has migrated to another mollysocket
/// instance, but we are still connected, causing an error 4409 on the other instance
const DELIVERY_CHECK_TIMEOUT: Duration = Duration::from_hours(1);

#[derive(Debug)]
pub struct Channels {
    ws_tx: Option<mpsc::UnboundedSender<tungstenite::Message>>,
    pub on_message_tx: Option<mpsc::UnboundedSender<u32>>,
    pub on_push_tx: Option<mpsc::UnboundedSender<u32>>,
    pub on_reconnection_tx: Option<mpsc::UnboundedSender<u32>>,
}

impl Channels {
    fn none() -> Self {
        Self {
            ws_tx: None,
            on_message_tx: None,
            on_push_tx: None,
            on_reconnection_tx: None,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    /// We got:
    /// \* a 403
    /// \* or a ws response ConnectedElseWhere and our push service isn't valid anymore
    /// => the registration has migrated
    RegistrationRemoved,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub struct SignalWebSocket {
    creds: String,
    push_endpoint: url::Url,
    pub channels: Channels,
    push_instant: Arc<Mutex<Instant>>,
    last_keepalive: Arc<Mutex<Instant>>,
}

#[async_trait(?Send)]
impl WebSocketConnection for SignalWebSocket {
    fn get_url(&self) -> &str {
        &config::get_ws_endpoint()
    }

    fn get_creds(&self) -> &str {
        &self.creds
    }

    fn get_websocket_tx(&self) -> &Option<mpsc::UnboundedSender<tungstenite::Message>> {
        &self.channels.ws_tx
    }

    fn set_websocket_tx(&mut self, tx: Option<mpsc::UnboundedSender<tungstenite::Message>>) {
        self.channels.ws_tx = tx;
    }

    fn get_last_keepalive(&self) -> Arc<Mutex<Instant>> {
        Arc::clone(&self.last_keepalive)
    }

    async fn on_message(&self, message: WebSocketMessage) -> Result<()> {
        if let Some(type_int) = message.r#type {
            if let Ok(type_) = Type::try_from(type_int) {
                match type_ {
                    Type::Response => self.on_response(message.response),
                    Type::Request => {
                        let _ = self.on_request(message.request).await?;
                    }
                    _ => (),
                };
            }
        }
        Ok(())
    }
}

impl SignalWebSocket {
    pub fn new<'a, 'b: 'a>(
        uuid: &str,
        device_id: u32,
        password: &str,
        push_endpoint: &str,
    ) -> Result<Self> {
        let push_endpoint = url::Url::parse(&push_endpoint)?;
        Ok(Self {
            creds: format!("{}.{}:{}", uuid, device_id, password),
            push_endpoint,
            channels: Channels::none(),
            push_instant: Arc::new(Mutex::new(
                Instant::now().checked_sub(PUSH_TIMEOUT).unwrap(),
            )),
            last_keepalive: Arc::new(Mutex::new(Instant::now())),
        })
    }

    pub async fn connection_loop(&mut self) -> Result<()> {
        let mut count = 0;
        loop {
            let instant = Instant::now();
            {
                let mut keepalive = self.last_keepalive.lock().unwrap();
                *keepalive = Instant::now();
            }
            if let Err(e) = self.connect(tls::build_tls_connector()?).await {
                if let Some(Error::RegistrationRemoved) = e.downcast_ref::<Error>() {
                    log::debug!("connection_loop: got RegistrationRemoved.");
                    return Err(eyre!(Error::RegistrationRemoved));
                } else if let Some(tungstenite::Error::Http(resp)) =
                    e.downcast_ref::<tungstenite::Error>()
                {
                    if resp.status() == 403 {
                        log::debug!("connection_loop: got HTTP error 403 (linked device creds aren't valid anymore).");
                        return Err(eyre!(Error::RegistrationRemoved));
                    } else {
                        log::debug!("HTTP error: {:?}", e);
                    }
                } else if let Some(websocket_connection::Error::ConnectedElseWhere) =
                    e.downcast_ref::<websocket_connection::Error>()
                {
                    log::debug!("connection_loop: got ConnectedElseWhere.");
                    // we try to push a simple json {"code": 4409}, if we receive a 403, 404 or 410:
                    // then the registration should be handled as removed (like a 403)
                    let _ = self.push_delivery_check().await?;
                } else {
                    log::debug!("Connection error: {:?}", e);
                }
            }
            if let Some(duration) = Instant::now().checked_duration_since(instant) {
                if duration > Duration::from_secs(60) {
                    count = 0;
                }
            }
            if let Some(tx) = &self.channels.on_reconnection_tx {
                let _ = tx.unbounded_send(1);
            }
            count += 1;
            log::info!("Retrying to connect in {}0 seconds.", count);
            time::sleep(Duration::from_secs(count * 10)).await;
        }
    }

    fn on_response(&self, response: Option<WebSocketResponseMessage>) {
        log::debug!("New response");
        if response.is_some() {
            let mut keepalive = self.last_keepalive.lock().unwrap();
            *keepalive = Instant::now();
        }
    }

    /**
     * That's when we must send a notification
     */
    async fn on_request(&self, request: Option<WebSocketRequestMessage>) -> Result<()> {
        log::debug!("New request");
        if let Some(request) = request {
            if let Some(envelope) = self.request_to_envelope(request).await {
                if let Some(tx) = &self.channels.on_message_tx {
                    let _ = tx.unbounded_send(1);
                }
                if self.waiting_timeout_reached() {
                    if envelope.urgent() {
                        let _ = self.send_push().await?;
                    }
                } else {
                    log::debug!("The waiting timeout is not reached: the request is ignored.");
                }
            }
        }
        Ok(())
    }

    /**
     * Extract [`Envelope`] from [`request`] and send response to server.
     */
    async fn request_to_envelope(&self, request: WebSocketRequestMessage) -> Option<Envelope> {
        // dbg!(&request.path);
        let response = self.create_websocket_response(&request);
        // dbg!(&response);
        if self.is_signal_service_envelope(&request) {
            self.send_response(response).await;
            return match request.body {
                None => Some(Envelope {
                    r#type: None,
                    source_service_id: None,
                    source_device: None,
                    destination_service_id: None,
                    timestamp: None,
                    content: None,
                    server_guid: None,
                    server_timestamp: None,
                    urgent: Some(false),
                    updated_pni: None,
                    story: None,
                    reporting_token: None,
                }),
                Some(body) => Envelope::decode(&body[..]).ok(),
            };
        }
        None
    }

    fn is_signal_service_envelope(
        &self,
        WebSocketRequestMessage {
            verb,
            path,
            body: _,
            headers: _,
            id: _,
        }: &WebSocketRequestMessage,
    ) -> bool {
        if let Some(verb) = verb {
            if let Some(path) = path {
                return verb.eq("PUT") && path.eq("/api/v1/message");
            }
        }
        false
    }

    fn create_websocket_response(
        &self,
        request: &WebSocketRequestMessage,
    ) -> WebSocketResponseMessage {
        if self.is_signal_service_envelope(request) {
            return WebSocketResponseMessage {
                id: request.id,
                status: Some(200),
                message: Some(String::from("OK")),
                headers: Vec::new(),
                body: None,
            };
        }
        WebSocketResponseMessage {
            id: request.id,
            status: Some(400),
            message: Some(String::from("Unknown")),
            headers: Vec::new(),
            body: None,
        }
    }

    async fn send_push(&self) -> Result<()> {
        log::debug!("Sending the notification.");
        {
            let mut instant = self.push_instant.lock().unwrap();
            *instant = Instant::now();
        }

        let url = self.push_endpoint.clone();
        let res = post_allowed(url, &json!({"urgent": true}), Some("mollysocket")).await;
        if let Some(tx) = &self.channels.on_push_tx {
            let _ = tx.unbounded_send(1);
        }
        self.assert_push_response(res)
    }

    /// If we received an error 4409 "connected elsewhere", we send a "delivery check" push notif:
    /// \* if we receive a 403/404/410, then the endpoint as been removed and we should
    /// delete the registration
    /// \* else, the other instance is probably the one that need to unregister, which will disable the registration
    /// with the same mechanism
    /// \* if the 2 instances run with a valid endpoint, the user needs to rotate the linked device
    ///
    /// We send the test notification, only if we didn't send one during the last [DELIVERY_CHECK_TIMEOUT] period
    /// to avoid spamming the client
    async fn push_delivery_check(&self) -> Result<()> {
        {
            let mut instant = self.push_instant.lock().unwrap();
            if instant.elapsed() > DELIVERY_CHECK_TIMEOUT {
                log::info!("push_delivery_check: We send a notification recently, no need to push a delivery check.");
                return Ok(());
            }
            // We set the last push notif to now - PUSH_TIMEOUT, so if a push notification arrives
            // between now and now + PUSH_TIMEOUT, it can wake the client correctly
            // We fallback to now, in case of error, but it shouldn't fail.
            *instant = Instant::now()
                .checked_sub(PUSH_TIMEOUT)
                .unwrap_or(Instant::now());
        }
        let url = self.push_endpoint.clone();
        let res = post_allowed(url, &json!({"code": 4409}), Some("4409")).await;
        log::trace!("{:?}", res);
        self.assert_push_response(res)
    }

    fn assert_push_response(&self, res: Result<reqwest::Response>) -> Result<()> {
        match res {
            Err(err) => {
                if let Some(e) = err.downcast_ref::<reqwest::Error>() {
                    match e.status() {
                        Some(StatusCode::FORBIDDEN)
                        | Some(StatusCode::NOT_FOUND)
                        | Some(StatusCode::GONE) => {
                            log::debug!("Got response with status={:?}", e.status());
                            return Err(err.wrap_err(eyre!(Error::RegistrationRemoved)));
                        }
                        _ => (),
                    }
                }
            }
            Ok(resp) => match resp.status() {
                StatusCode::FORBIDDEN | StatusCode::NOT_FOUND | StatusCode::GONE => {
                    log::debug!("Got response with status={:?}", resp.status());
                    return Err(eyre!(Error::RegistrationRemoved));
                }
                _ => (),
            },
        }
        Ok(())
    }

    fn waiting_timeout_reached(&self) -> bool {
        let instant = self.push_instant.lock().unwrap();
        instant.elapsed() > PUSH_TIMEOUT
    }
}
