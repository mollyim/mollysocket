use prost::Message;
use tokio_tungstenite::tungstenite;
use websocket_connection::{
    websocket_message::{webSocketMessage::Type, WebSocketMessage, WebSocketRequestMessage},
    WebSocketConnection,
};

mod tls;
mod websocket_connection;

pub async fn connect(connect_addr: &str) {
    let mut ws_connection = WebSocketConnection::new();

    ws_connection
        .connect(connect_addr, tls::build_tls_connector().unwrap())
        .await;
}

fn on_message(message: tungstenite::Message) {
    let data = message.into_data();
    let ws_message = WebSocketMessage::decode(&data[..]).unwrap();
    dbg!(&ws_message);
    match ws_message.r#type {
        Some(type_int) => match Type::from_i32(type_int) {
            Some(Type::RESPONSE) => (),
            Some(Type::REQUEST) => on_request(ws_message.request),
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
