use futures_channel::mpsc;
use futures_util::StreamExt;
use native_tls::TlsConnector;
use prost::Message;
use std::{thread, time::Duration};
use tokio_tungstenite::{
    tungstenite::{self, client::IntoClientRequest},
    Connector::NativeTls,
};

use websocket_message::{webSocketMessage::Type, WebSocketMessage, WebSocketRequestMessage};

pub mod websocket_message;

pub struct WebSocketConnection {
    tx: Option<mpsc::UnboundedSender<tungstenite::Message>>,
}

impl Clone for WebSocketConnection {
    fn clone(&self) -> WebSocketConnection {
        WebSocketConnection {
            tx: self.tx.clone(),
        }
    }
}

impl WebSocketConnection {
    pub fn new() -> WebSocketConnection {
        WebSocketConnection { tx: None }
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
                    self.on_message(message);
                }
            })
        };

        let ka_connection = self.clone();

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(3));
            println!("> Sending Keepalive");
            ka_connection.send_keepalive();
        });

        // handle websocket
        let _ = futures::join!(msg_handle, ws_handle);
    }

    pub fn send(&self, message: websocket_message::WebSocketMessage) {
        if let Some(tx) = self.tx.as_ref() {
            let mut buf = Vec::new();
            buf.reserve(message.encoded_len());
            message.encode(&mut buf).unwrap();
            tx.unbounded_send(tungstenite::Message::Binary(buf))
                .unwrap();
        }
    }

    fn on_message(&self, message: tungstenite::Message) {
        let data = message.into_data();
        let ws_message = match WebSocketMessage::decode(&data[..]) {
            Err(_) => {
                println!("Can't decode msg");
                return ();
            }
            Ok(msg) => msg,
        };
        // dbg!(&ws_message);
        match ws_message.r#type {
            Some(type_int) => match Type::from_i32(type_int) {
                Some(Type::RESPONSE) => (),
                Some(Type::REQUEST) => self.on_request(ws_message.request),
                _ => (),
            },
            None => (),
        };
    }

    /**
     * That's when we must send a notification
     */
    fn on_request(&self, request: Option<WebSocketRequestMessage>) {
        if let Some(request) = request {
            let _headers = request.headers;
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
    // pub fn set_on_message(&self, on_message: &dyn Fn(tungstenite::Message)) {
    //     loop {
    //         on_message(self.in_rx.recv().unwrap());
    //     }
    // }
}
