use async_trait::async_trait;
use eyre::Result;
use futures_channel::mpsc;
use futures_util::{pin_mut, select, FutureExt, SinkExt, StreamExt};
use native_tls::TlsConnector;
use prost::Message;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tokio::time;
use tokio_tungstenite::{
    tungstenite::{self, client::IntoClientRequest},
    Connector::NativeTls,
};

use super::websocket_message::{
    webSocketMessage::Type, WebSocketMessage, WebSocketRequestMessage, WebSocketResponseMessage,
};

const KEEPALIVE: Duration = Duration::from_secs(30);
const KEEPALIVE_TIMEOUT: Duration = Duration::from_secs(40);

#[async_trait(?Send)]
pub trait WebSocketConnection {
    fn get_url(&self) -> &url::Url;
    fn get_tx(&self) -> &Option<mpsc::UnboundedSender<tungstenite::Message>>;
    fn set_tx(&mut self, tx: Option<mpsc::UnboundedSender<tungstenite::Message>>);
    fn get_last_keepalive(&self) -> Arc<Mutex<Instant>>;
    async fn on_message(&self, message: WebSocketMessage);

    async fn connect(&mut self, tls_connector: TlsConnector) -> Result<()> {
        let mut request = self.get_url().into_client_request()?;

        request
            .headers_mut()
            .insert("X-Signal-Agent", http::HeaderValue::from_static("\"OWA\""));

        let (ws_stream, _) = tokio_tungstenite::connect_async_tls_with_config(
            request,
            None,
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
        self.set_tx(Some(tx));

        // handlers
        let to_ws_handle = rx.map(Ok).forward(ws_write).fuse();

        let from_ws_handle = ws_read
            .for_each(|message| async {
                log::debug!("New message");
                if let Ok(message) = message {
                    self.handle_message(message).await;
                }
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
            _ = to_ws_handle => log::warn!("Messages finished"),
            _ = from_ws_handle => log::warn!("Websocket finished"),
            _ = from_keepalive_handle => log::warn!("Keepalive finished"),
            _ = to_keepalive_handle => log::warn!("Keepalive finished"),
        );
        Ok(())
    }

    async fn handle_message(&self, message: tungstenite::Message) {
        let data = message.into_data();
        let ws_message = match WebSocketMessage::decode(&data[..]) {
            Ok(msg) => msg,
            Err(e) => {
                log::error!("Failed to decode protobuf: {}", e);
                return ();
            }
        };
        self.on_message(ws_message).await;
    }

    async fn send_message(&self, message: WebSocketMessage) {
        if let Some(mut tx) = self.get_tx().as_ref() {
            let bytes = message.encode_to_vec();
            tx.send(tungstenite::Message::binary(bytes)).await.unwrap();
        }
    }

    async fn send_response(&self, response: WebSocketResponseMessage) {
        let message = WebSocketMessage {
            r#type: Some(Type::RESPONSE as i32),
            response: Some(response),
            request: None,
        };
        self.send_message(message).await;
    }

    async fn send_keepalive(&self) {
        log::debug!("send_keepalive");
        let message = WebSocketMessage {
            r#type: Some(Type::REQUEST as i32),
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
        let last_keepalive = self.get_last_keepalive();
        loop {
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
