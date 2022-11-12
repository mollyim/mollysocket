use crate::signalwebsocket::SignalWebSocket;
use std::env::{self, Args};

fn usage() {
    println!(
        "Usage: {} oneshot wss://signal.server.tld/path https://push.server.ltd/id",
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
    };
    let push_endpoint = match argv.get(1) {
        Some(argv1) => argv1,
        None => {
            usage();
            return;
        }
    };
    let _ = SignalWebSocket::new(connect_addr.clone(), push_endpoint.clone())
        .connection_loop()
        .await;
}
