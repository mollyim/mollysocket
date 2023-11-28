use crate::ws::SignalWebSocket;

/* 
Usage: {} oneshot wss://signal.server.tld/path https://push.server.ltd/id
Strategies:
  rest        Send all messages
  websocket   Send all messages at least 5 seconds apart
*/

pub async fn oneshot(connect_addr: &str, push_endpoint: &str) {
    let _ = SignalWebSocket::new(
            connect_addr.to_string(),
            push_endpoint.to_string(),
        )
        .unwrap()
        .connection_loop()
        .await;
}
