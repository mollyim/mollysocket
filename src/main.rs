use lazy_static::lazy_static;
use std::{env, process};

use config::Config;
use signalwebsocket::SignalWebSocket;

mod config;
mod signalwebsocket;

lazy_static! {
    #[derive(Debug)]
    static ref CONFIG: Config = Config::load();
}

fn usage() {
    println!(
        "Usage: {} wss://signal.server.tld/path https://push.server.ltd/id",
        env::args().nth(0).unwrap()
    );
}

#[tokio::main]
async fn main() {
    env_logger::init();
    // dbg!(&*CONFIG);
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
