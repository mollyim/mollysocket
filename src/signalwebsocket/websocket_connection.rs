use futures_channel::mpsc;
use futures_util::StreamExt;
use native_tls::TlsConnector;
use prost::Message;
use std::{sync::Arc, thread, time::Duration};
use tokio_tungstenite::{
    tungstenite::{self, client::IntoClientRequest},
    Connector::NativeTls,
};

use websocket_message::{
    webSocketMessage::Type, WebSocketMessage, WebSocketRequestMessage, WebSocketResponseMessage,
};

pub mod websocket_message;

pub struct WebSocketConnection {
    tx: Option<mpsc::UnboundedSender<tungstenite::Message>>,
    fn_on_message: Option<Arc<dyn Fn(&Self, WebSocketMessage) + Send + Sync>>,
}

impl Clone for WebSocketConnection {
    fn clone(&self) -> WebSocketConnection {
        WebSocketConnection {
            tx: self.tx.clone(),
            fn_on_message: match &self.fn_on_message {
                Some(f) => Some(Arc::clone(&f)),
                None => None,
            },
        }
    }
}

impl WebSocketConnection {
    pub fn new() -> WebSocketConnection {
        WebSocketConnection {
            tx: None,
            fn_on_message: None,
        }
    }

    pub fn set_on_message(
        &mut self,
        fn_on_message: Option<Arc<dyn Fn(&Self, WebSocketMessage) + Send + Sync>>,
    ) {
        self.fn_on_message = fn_on_message;
    }

    pub async fn connect(&mut self, connect_addr: &str, tls_connector: TlsConnector) {
        let mut request = url::Url::parse(&connect_addr)
            .expect("Failed to parse URL")
            .into_client_request()
            .unwrap();

        request
            .headers_mut()
            .insert("X-Signal-Agent", http::HeaderValue::from_static("\"OWA\""));

        let (ws_stream, _) = tokio_tungstenite::connect_async_tls_with_config(
            request,
            None,
            Some(NativeTls(tls_connector)),
        )
        .await
        .expect("Failed to connect");

        println!("WebSocket handshake has been successfully completed");

        // Websocket I/O
        let (ws_write, ws_read) = ws_stream.split();

        // channel to websocket ws_write
        let (tx, rx) = mpsc::unbounded();
        self.tx = Some(tx);
        let msg_handle = rx.map(Ok).forward(ws_write);

        let ws_handle = {
            ws_read.for_each(|message| async {
                println!("> New message");
                if let Ok(message) = message {
                    if let Some(on_message) = &self.fn_on_message {
                        let data = message.into_data();
                        let ws_message = match WebSocketMessage::decode(&data[..]) {
                            Err(_) => {
                                println!("Can't decode msg");
                                return ();
                            }
                            Ok(msg) => msg,
                        };
                        on_message(&self, ws_message);
                    }
                }
            })
        };

        let ka_connection = self.clone();

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(30));
            println!("> Sending Keepalive");
            ka_connection.send_keepalive();
        });

        // handle websocket
        let _ = futures::join!(msg_handle, ws_handle);
    }

    fn send(&self, message: WebSocketMessage) {
        if let Some(tx) = self.tx.as_ref() {
            let mut buf = Vec::new();
            buf.reserve(message.encoded_len());
            message.encode(&mut buf).unwrap();
            tx.unbounded_send(tungstenite::Message::Binary(buf))
                .unwrap();
        }
    }

    fn send_keepalive(&self) {
        println!("send_keepalive");
        let message = WebSocketMessage {
            r#type: Some(Type::REQUEST as i32),
            response: None,
            request: Some(WebSocketRequestMessage {
                verb: Some(String::from("GET")),
                path: Some(String::from("/v1/keepalive")),
                body: None,
                headers: Vec::new(),
                id: None,
            }),
        };
        self.send(message);
    }

    pub fn send_response(&self, response: WebSocketResponseMessage) {
        let message = WebSocketMessage {
            r#type: Some(Type::RESPONSE as i32),
            response: Some(response),
            request: None,
        };
        self.send(message);
    }
}
