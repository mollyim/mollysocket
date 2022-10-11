use std::sync::Arc;
use websocket_connection::{
    websocket_message::{webSocketMessage::Type, WebSocketMessage, WebSocketRequestMessage},
    WebSocketConnection,
};

mod tls;
mod websocket_connection;

pub async fn connect(connect_addr: &str) {
    let mut ws_connection = WebSocketConnection::new();

    ws_connection.set_on_message(Some(Arc::new(&on_message)));

    ws_connection
        .connect(connect_addr, tls::build_tls_connector().unwrap())
        .await;
}

fn on_message(connection: &WebSocketConnection, message: WebSocketMessage) {
    // dbg!(&message);
    match message.r#type {
        Some(type_int) => match Type::from_i32(type_int) {
            Some(Type::RESPONSE) => (),
            Some(Type::REQUEST) => on_request(message.request),
            _ => (),
        },
        None => (),
    };
}

/**
* That's when we must send a notification
*/
fn on_request(request: Option<WebSocketRequestMessage>) {
    if let Some(request) = request {
        let _headers = request.headers;
    }
}
