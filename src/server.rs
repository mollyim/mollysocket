use crate::{
    db::{Connection, MollySocketDb},
    error::Error,
    signalwebsocket::SignalWebSocket,
    CONFIG,
};
use futures_channel::mpsc;
use futures_util::join;
use futures_util::{future::join_all, select, FutureExt, StreamExt};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tokio_tungstenite::tungstenite;

mod web;

struct LoopRef {
    uuid: String,
    tx: mpsc::UnboundedSender<bool>,
}

lazy_static! {
    static ref REFS: Arc<Mutex<Vec<LoopRef>>> = Arc::new(Mutex::new(vec![]));
    static ref DB: MollySocketDb = MollySocketDb::new().unwrap();
}

pub async fn run() {
    let mut connections = DB.list().unwrap();
    let loops: Vec<_> = connections
        .iter_mut()
        .map(|co| connection_loop(co).fuse())
        .collect();

    let web = web::launch().fuse();

    join!(join_all(loops), web);
}

async fn connection_loop(co: &mut Connection) {
    if co.forbidden {
        log::info!("Ignoring connection for {}", &co.uuid);
        return;
    }
    log::info!("Starting connection for {}", &co.uuid);
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
        res = socket.connection_loop().fuse() => handle_connection_closed(res, co),
        _ = rx.next().fuse() => log::info!("Connection killed"),
    );
    let mut refs = REFS.lock().unwrap();
    if let Some(i_ref) = refs.iter().position(|l_ref| l_ref.uuid.eq(&co.uuid)) {
        refs.remove(i_ref);
    }
}

fn handle_connection_closed(res: Result<(), Error>, co: &mut Connection) {
    log::debug!("Connection closed.");
    if let Err(Error::Ws(e)) = res {
        if let tungstenite::Error::Http(resp) = e {
            let status = resp.status();
            log::info!("Connection for {} closed with status: {}", &co.uuid, status);
            if status == 403 {
                co.forbidden = true;
                let _ = DB.add(co);
            }
        }
    }
}

async fn kill(uuid: &str) {
    let refs = REFS.lock().unwrap();
    if let Some(l_ref) = refs.iter().find(|&l_ref| l_ref.uuid.eq(uuid)) {
        let _ = l_ref.tx.clone().unbounded_send(true);
    }
}
