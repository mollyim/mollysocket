use crate::db::{Connection, MollySocketDb};
use crate::signalwebsocket::SignalWebSocket;
use crate::web;
use crate::CONFIG;
use futures_channel::mpsc;
use futures_util::join;
use futures_util::{future::join_all, select, FutureExt, StreamExt};
use lazy_static::lazy_static;
use std::{
    env::{self, Args},
    sync::{Arc, Mutex},
};

struct LoopRef {
    uuid: String,
    tx: mpsc::UnboundedSender<bool>,
}

lazy_static! {
    static ref REFS: Arc<Mutex<Vec<LoopRef>>> = Arc::new(Mutex::new(vec![]));
}

fn usage() {
    println!(
        "
Usage: {} server
",
        env::args().nth(0).unwrap()
    );
}

pub async fn server(mut args: Args) {
    if args.any(|arg| arg == "--help" || arg == "-h") {
        return usage();
    };
    let connections = MollySocketDb::new().unwrap().list().unwrap();
    let loops: Vec<_> = connections
        .iter()
        .map(|co| connection_loop(co).fuse())
        .collect();

    let web = web::launch().fuse();

    join!(join_all(loops), web);
}

async fn connection_loop(co: &Connection) {
    let (tx, mut rx) = mpsc::unbounded();
    {
        REFS.lock().unwrap().push(LoopRef {
            uuid: co.uuid.clone(),
            tx,
        });
    }
    let mut socket = SignalWebSocket::new(
        CONFIG.get_ws_endpoint(&co.uuid, co.device_id, &co.password),
        co.endpoint.clone(),
    );
    select!(
        _ = socket.connection_loop().fuse() => log::info!("Connection finished"),
        _ = rx.next().fuse() => log::info!("Connection closed"),
    )
}

async fn kill(uuid: &str) {
    let refs = REFS.lock().unwrap();
    if let Some(l_ref) = refs.iter().find(|&l_ref| l_ref.uuid.eq(uuid)) {
        let _ = l_ref.tx.clone().unbounded_send(true);
    }
}
