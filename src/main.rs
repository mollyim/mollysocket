use std::{env, process};

use crate::signalwebsocket::SignalWebSocket;

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
    SignalWebSocket::new(connect_addr.clone(), push_endpoint.clone())
        .connection_loop()
        .await;
}
