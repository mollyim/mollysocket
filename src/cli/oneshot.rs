use crate::{db::Strategy, ws::SignalWebSocket};
use std::{
    env::{self, Args},
    str::FromStr,
};

fn usage() {
    println!(
        "
Usage: {} oneshot wss://signal.server.tld/path https://push.server.ltd/id [strategy]

Strategies:
  rest        Send all messages
  websocket   Send all messages at least 5 seconds apart
",
        env::args().nth(0).unwrap()
    );
}

pub async fn oneshot(args: Args) {
    let argv: Vec<String> = args.collect();
    if argv.iter().any(|arg| arg == "--help" || arg == "-h") {
        usage();
        return;
    }
    let connect_addr = match argv.get(0) {
        Some(argv1) => argv1,
        None => {
            usage();
            return;
        }
    }
    .clone();
    let push_endpoint = match argv.get(1) {
        Some(argv2) => argv2,
        None => {
            usage();
            return;
        }
    }
    .clone();
    let strategy = match argv.get(2) {
        Some(argv3) => Strategy::from_str(argv3).unwrap_or(Strategy::Websocket),
        None => {
            usage();
            return;
        }
    };

    let _ = SignalWebSocket::new(connect_addr, push_endpoint, strategy)
        .unwrap()
        .connection_loop()
        .await;
}
