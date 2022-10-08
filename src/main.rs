use std::env;
use std::process;

use crate::signalwebsocket::connect;

pub mod signalwebsocket;

fn usage() {
    println!(
        "Usage: {} wss://your.server.tld/path",
        env::args().nth(0).unwrap()
    );
}

#[tokio::main]
async fn main() {
    let connect_addr = env::args().nth(1).unwrap_or_else(|| {
        usage();
        process::exit(0)
    });
    connect(&connect_addr).await;
}
