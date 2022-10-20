use std::env;
use std::process;

use crate::signalwebsocket::{tls, websocket_connection::WebSocketConnection, SignalWebSocket};

pub mod signalwebsocket;

fn usage() {
    println!(
        "Usage: {} wss://signal.server.tld/path https://push.server.ltd/id",
        env::args().nth(0).unwrap()
    );
}

#[tokio::main]
async fn main() {
    let connect_addr = env::args().nth(1).unwrap_or_else(|| {
        usage();
        process::exit(0)
    });
    let push_endpoint = env::args().nth(2).unwrap_or_else(|| {
        usage();
        process::exit(0)
    });
    SignalWebSocket::new(connect_addr, push_endpoint)
        .connect(tls::build_tls_connector().unwrap())
        .await
}
