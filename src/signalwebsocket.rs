use async_std::task;
use async_trait::async_trait;
use futures_channel::mpsc;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio_tungstenite::tungstenite;
use websocket_connection::{
    websocket_message::{
        webSocketMessage::Type, WebSocketMessage, WebSocketRequestMessage, WebSocketResponseMessage,
    },
    WebSocketConnection,
};

pub mod tls;
pub mod websocket_connection;

const SERVER_DELIVERED_TIMESTAMP_HEADER: &str = "X-Signal-Timestamp";
const PUSH_TIMEOUT: Duration = Duration::from_secs(5);

pub struct SignalWebSocket {
    connect_addr: url::Url,
    push_endpoint: url::Url,
    tx: Option<mpsc::UnboundedSender<tungstenite::Message>>,
    push_instant: Arc<Mutex<Instant>>,
    last_keepalive: Arc<Mutex<Instant>>,
}

#[async_trait(?Send)]
impl WebSocketConnection for SignalWebSocket {
    fn get_url(&self) -> &url::Url {
        &self.connect_addr
    }

    fn get_tx(&self) -> &Option<mpsc::UnboundedSender<tungstenite::Message>> {
        &self.tx
    }

    fn set_tx(&mut self, tx: Option<mpsc::UnboundedSender<tungstenite::Message>>) {
        self.tx = tx
    }

    fn get_last_keepalive(&self) -> Arc<Mutex<Instant>> {
        Arc::clone(&self.last_keepalive)
    }

    async fn on_message(&self, message: WebSocketMessage) {
        match message.r#type {
            Some(type_int) => match Type::from_i32(type_int) {
                Some(Type::RESPONSE) => self.on_response(message.response),
                Some(Type::REQUEST) => self.on_request(message.request).await,
                _ => (),
            },
            None => (),
        };
    }
}

impl SignalWebSocket {
    pub fn new(connect_addr: String, push_endpoint: String) -> Self {
        let connect_addr = url::Url::parse(&connect_addr).expect("Cannot parse websocket url");
        let push_endpoint = url::Url::parse(&push_endpoint).expect("Cannot parse endpoint url");
        Self {
            connect_addr,
            push_endpoint,
            tx: None,
            push_instant: Arc::new(Mutex::new(
                Instant::now().checked_sub(PUSH_TIMEOUT).unwrap(),
            )),
            last_keepalive: Arc::new(Mutex::new(Instant::now())),
        }
    }

    pub async fn connection_loop(&mut self) {
        let mut count = 0;
        loop {
            let instant = Instant::now();
            let mut keepalive = self.last_keepalive.lock().unwrap();
            *keepalive = Instant::now();
            drop(keepalive);
            self.connect(tls::build_tls_connector().unwrap()).await;
            if let Some(duration) = Instant::now().checked_duration_since(instant) {
                if duration > Duration::from_secs(60) {
                    count = 0;
                }
            }
            count += 1;
            println!("> Retrying in {}0 secondes.", count);
            task::sleep(Duration::from_secs(count * 10)).await;
        }
    }

    fn on_response(&self, response: Option<WebSocketResponseMessage>) {
        println!("  > New response");
        if let Some(_) = response {
            let mut last_keepalive = self.last_keepalive.lock().unwrap();
            *last_keepalive = Instant::now()
        }
    }

    /**
     * That's when we must send a notification
     */
    async fn on_request(&self, request: Option<WebSocketRequestMessage>) {
        println!("  > New request");
        if let Some(request) = request {
            if self.read_or_empty(request) {
                if self.over_timeout() {
                    self.notify().await;
                } else {
                    println!("  > Push timeout not past. Ignoring request");
                }
            }
        }
    }

    fn read_or_empty(&self, request: WebSocketRequestMessage) -> bool {
        dbg!(&request.path);
        let response = self.create_websocket_response(&request);
        // dbg!(&response);
        if self.is_signal_service_envelope(&request) {
            let timestamp: u64 = match self.find_header(&request) {
                Some(timestamp) => timestamp.parse().unwrap(),
                None => 0,
            };
            self.send_response(response);
            return true;
        }
        false
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

    fn is_socket_empty_request(
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
                return verb.eq("PUT") && path.eq("/api/v1/queue/empty");
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

    fn find_header(&self, message: &WebSocketRequestMessage) -> Option<String> {
        if message.headers.len() == 0 {
            return None;
        }
        let mut header_iter = message.headers.iter().filter_map(|header| {
            if header
                .to_lowercase()
                .starts_with(SERVER_DELIVERED_TIMESTAMP_HEADER)
            {
                let mut split = header.split(":");
                if let Some(header_name) = split.next() {
                    if let Some(header_value) = split.next() {
                        if header_name
                            .trim()
                            .eq_ignore_ascii_case(SERVER_DELIVERED_TIMESTAMP_HEADER)
                        {
                            return Some(String::from(header_value.to_lowercase().trim()));
                        }
                    }
                }
            }
            None
        });
        header_iter.next()
    }

    async fn notify(&self) {
        println!("  > Notifying");

        let mut instant = self.push_instant.lock().unwrap();
        *instant = Instant::now();

        let url = self.push_endpoint.clone();
        let _ = reqwest::Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .body("{\"type\":\"request\"}")
            .send()
            .await;
    }

    fn over_timeout(&self) -> bool {
        let instant = self.push_instant.lock().unwrap();
        instant.elapsed() > PUSH_TIMEOUT
    }
}
