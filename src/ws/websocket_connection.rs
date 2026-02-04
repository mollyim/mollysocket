use async_trait::async_trait;
use base64::{prelude::BASE64_STANDARD, Engine};
use eyre::Result;
use futures_channel::mpsc;
use futures_util::{pin_mut, select, FutureExt, SinkExt, StreamExt, TryStreamExt};
use native_tls::TlsConnector;
use prost::Message;
use std::{
    fmt::{Display, Formatter},
    sync::{Arc, Mutex},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tokio::time;
use tokio_tungstenite::{
    tungstenite::{
        self,
        protocol::{self, frame::coding::CloseCode, CloseFrame},
        ClientRequestBuilder, Error as TError,
    },
    Connector::NativeTls,
};

use super::proto_websocketresources::{
    web_socket_message::Type, WebSocketMessage, WebSocketRequestMessage, WebSocketResponseMessage,
};

const KEEPALIVE: Duration = Duration::from_secs(30);
const KEEPALIVE_TIMEOUT: Duration = Duration::from_secs(40);

#[derive(Debug)]
pub enum Error {
    #[allow(dead_code)]
    Ws(TError),
    ConnectedElseWhere,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

#[async_trait(?Send)]
pub trait WebSocketConnection {
    fn get_url(&self) -> &str;
    /// return "login:password"
    fn get_creds(&self) -> &str;
    fn get_websocket_tx(&self) -> &Option<mpsc::UnboundedSender<tungstenite::Message>>;
    fn set_websocket_tx(&mut self, tx: Option<mpsc::UnboundedSender<tungstenite::Message>>);
    fn get_last_keepalive(&self) -> Arc<Mutex<Instant>>;
    async fn on_message(&self, message: WebSocketMessage);

    /// Connect to the server and handle messages
    /// Returns HTTP Error, or ConnectedElseWhere or () if disconnected normally
    async fn connect(&mut self, tls_connector: TlsConnector) -> Result<()> {
        let request = ClientRequestBuilder::new(self.get_url().parse()?)
            .with_header("X-Signal-Agent", "\"OWA\"")
            .with_header(
                "Authorization",
                format!("Basic {}", BASE64_STANDARD.encode(self.get_creds())),
            );

        let (ws_stream, _) = tokio_tungstenite::connect_async_tls_with_config(
            request,
            None,
            false,
            Some(NativeTls(tls_connector)),
        )
        .await?;

        log::info!("WebSocket handshake has been successfully completed");

        // Websocket I/O
        let (ws_write, ws_read) = ws_stream.split();
        // channel to websocket ws_write
        let (tx, rx) = mpsc::unbounded();
        // other channels: msg, keepalive, abort
        let (timer_tx, timer_rx) = mpsc::unbounded();

        // Saving to socket Sender
        self.set_websocket_tx(Some(tx));

        // handlers
        let to_ws_handle = rx.map(Ok).forward(ws_write).fuse();

        let from_ws_handle = ws_read
            .map_err(Error::Ws)
            .try_for_each(|message| async {
                log::debug!("New message");
                log::trace!("{:?}", message);
                if message.is_close() {
                    if let protocol::Message::Close(Some(CloseFrame {
                        code: CloseCode::Library(4409),
                        reason: _,
                    })) = message
                    {
                        log::debug!("Websocket closed: connected elsewhere");
                        return Err(Error::ConnectedElseWhere);
                    } else {
                        log::debug!("Websocket closed normally")
                    }
                } else if message.is_binary() {
                    self.handle_message(message).await;
                }
                Ok(())
            })
            .fuse();

        let from_keepalive_handle = timer_rx
            .for_each(|_| async { self.send_keepalive().await })
            .fuse();

        let to_keepalive_handle = self.loop_keepalive(timer_tx).fuse();

        pin_mut!(
            to_ws_handle,
            from_ws_handle,
            from_keepalive_handle,
            to_keepalive_handle
        );

        // handle websocket
        select!(
            res = from_ws_handle => {
                log::warn!("Websocket finished: {:?}", res);
                if let Err(Error::ConnectedElseWhere) = res {
                    return Err(Error::ConnectedElseWhere.into());
                }
            },
            _ = to_ws_handle => log::warn!("Messages finished"),
            _ = from_keepalive_handle => log::warn!("Keepalive finished"),
            _ = to_keepalive_handle => log::warn!("Keepalive finished"),
        );
        Ok(())
    }

    async fn handle_message(&self, message: tungstenite::Message) {
        let data = message.into_data();
        let ws_message = match WebSocketMessage::decode(data) {
            Ok(msg) => msg,
            Err(e) => {
                log::error!("Failed to decode protobuf: {}", e);
                return;
            }
        };
        self.on_message(ws_message).await;
    }

    async fn send_message(&self, message: WebSocketMessage) {
        if let Some(mut tx) = self.get_websocket_tx().as_ref() {
            let bytes = message.encode_to_vec();
            tx.send(tungstenite::Message::binary(bytes)).await.unwrap();
        }
    }

    async fn send_response(&self, response: WebSocketResponseMessage) {
        let message = WebSocketMessage {
            r#type: Some(Type::Response as i32),
            response: Some(response),
            request: None,
        };
        self.send_message(message).await;
    }

    async fn send_keepalive(&self) {
        log::debug!("send_keepalive");
        let message = WebSocketMessage {
            r#type: Some(Type::Request as i32),
            response: None,
            request: Some(WebSocketRequestMessage {
                verb: Some(String::from("GET")),
                path: Some(String::from("/v1/keepalive")),
                body: None,
                headers: Vec::new(),
                id: Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                ),
            }),
        };
        self.send_message(message).await;
    }

    async fn loop_keepalive(&self, timer_tx: mpsc::UnboundedSender<bool>) {
        // Get the ref of last_keepalive
        let last_keepalive = self.get_last_keepalive();
        loop {
            // read last_keepalive
            if last_keepalive.lock().unwrap().elapsed() > KEEPALIVE_TIMEOUT {
                log::warn!("Did not receive the last keepalive: aborting.");
                break;
            }
            time::sleep(KEEPALIVE).await;
            log::debug!("Sending Keepalive");
            timer_tx.unbounded_send(true).unwrap();
        }
    }
}
