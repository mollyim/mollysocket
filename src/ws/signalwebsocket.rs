use async_trait::async_trait;
use eyre::Result;
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

const PUSH_TIMEOUT: Duration = Duration::from_secs(1);

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

    async fn on_message(&self, message: WebSocketMessage) {
        if let Some(type_int) = message.r#type {
            if let Ok(type_) = Type::try_from(type_int) {
                match type_ {
                    Type::Response => self.on_response(message.response),
                    Type::Request => self.on_request(message.request).await,
                    _ => (),
                };
            }
        }
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
                if let Some(tungstenite::Error::Http(resp)) = e.downcast_ref::<tungstenite::Error>()
                {
                    if resp.status() == 403 {
                        return Err(Error::RegistrationRemoved.into());
                    } else {
                        log::debug!("HTTP error: {:?}", e);
                    }
                } else if let Some(websocket_connection::Error::ConnectedElseWhere) =
                    e.downcast_ref::<websocket_connection::Error>()
                {
                    log::debug!("Connected elsewhere: informing client");
                    // we try to push a simple json {"code": 4409}, if we receive a 403, 404 or 410:
                    // then the registration should be handled as removed (like a 403)
                    let _ = self.push_on_connected_elsewhere().await?;
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
    async fn on_request(&self, request: Option<WebSocketRequestMessage>) {
        log::debug!("New request");
        if let Some(request) = request {
            if let Some(envelope) = self.request_to_envelope(request).await {
                if let Some(tx) = &self.channels.on_message_tx {
                    let _ = tx.unbounded_send(1);
                }
                if self.waiting_timeout_reached() {
                    if envelope.urgent() {
                        self.send_push().await;
                    }
                } else {
                    log::debug!("The waiting timeout is not reached: the request is ignored.");
                }
            }
        }
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

    async fn send_push(&self) {
        log::debug!("Sending the notification.");
        {
            let mut instant = self.push_instant.lock().unwrap();
            *instant = Instant::now();
        }

        let url = self.push_endpoint.clone();
        let _ = post_allowed(url, &json!({"urgent": true}), Some("mollysocket")).await;
        if let Some(tx) = &self.channels.on_push_tx {
            let _ = tx.unbounded_send(1);
        }
    }

    /// If we are connected elsewhere, we try to push a message to the endpoint:
    /// \* if we receive a 403/404/410, then the endpoint as been removed and we should
    /// delete the registration
    /// \* else, it may be useful for the client to rotate the linked device
    async fn push_on_connected_elsewhere(&self) -> Result<()> {
        let url = self.push_endpoint.clone();
        let res = post_allowed(url, &json!({"code": 4409}), Some("4409")).await;
        log::trace!("{:?}", res);
        match &res {
            Err(e) => {
                if let Some(e) = e.downcast_ref::<reqwest::Error>() {
                    match e.status() {
                        Some(StatusCode::FORBIDDEN)
                        | Some(StatusCode::NOT_FOUND)
                        | Some(StatusCode::GONE) => return Err(Error::RegistrationRemoved.into()),
                        _ => (),
                    }
                }
            }
            Ok(resp) => match resp.status() {
                StatusCode::FORBIDDEN | StatusCode::NOT_FOUND | StatusCode::GONE => {
                    return Err(Error::RegistrationRemoved.into())
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
