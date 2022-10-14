use std::sync::Arc;
use websocket_connection::{
    websocket_message::{
        webSocketMessage::Type, WebSocketMessage, WebSocketRequestMessage, WebSocketResponseMessage,
    },
    WebSocketConnection,
};

mod tls;
mod websocket_connection;

const SERVER_DELIVERED_TIMESTAMP_HEADER: &str = "X-Signal-Timestamp";

pub async fn connect(connect_addr: &str) {
    let mut ws_connection = WebSocketConnection::new();

    ws_connection.set_on_message(Some(Arc::new(&on_message)));

    ws_connection
        .connect(connect_addr, tls::build_tls_connector().unwrap())
        .await;
}

fn on_message(connection: &WebSocketConnection, message: WebSocketMessage) {
    match message.r#type {
        Some(type_int) => match Type::from_i32(type_int) {
            Some(Type::RESPONSE) => (),
            Some(Type::REQUEST) => on_request(connection, message.request),
            _ => (),
        },
        None => (),
    };
}

/**
* That's when we must send a notification
*/
fn on_request(connection: &WebSocketConnection, request: Option<WebSocketRequestMessage>) {
    if let Some(request) = request {
        read_or_empty(connection, request);
    }
}

fn read_or_empty(connection: &WebSocketConnection, request: WebSocketRequestMessage) -> bool {
    dbg!(&request.path);
    let response = create_websocket_response(&request);
    dbg!(&response);
    if is_signal_service_envelope(&request) {
        let timestamp: u64 = match find_header(&request) {
            Some(timestamp) => timestamp.parse().unwrap(),
            None => 0,
        };
        connection.send_response(response);
        return true;
    }
    false
}

fn is_signal_service_envelope(
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

fn create_websocket_response(request: &WebSocketRequestMessage) -> WebSocketResponseMessage {
    if is_signal_service_envelope(request) {
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

fn find_header(message: &WebSocketRequestMessage) -> Option<String> {
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
