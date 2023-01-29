use crate::{
    db::{Connection, Strategy},
    server::{DB, METRICS, REFS, TX},
    ws::SignalWebSocket,
    CONFIG,
};
use eyre::Result;
use futures_channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures_util::{future::join_all, join, select, Future, FutureExt, StreamExt};
use tokio_tungstenite::tungstenite;

pub struct LoopRef {
    uuid: String,
    tx: UnboundedSender<bool>,
}

pub type OptSender = Option<UnboundedSender<Connection>>;

pub async fn run() {
    let mut connections = DB.list().unwrap();
    let loops: Vec<_> = connections
        .iter_mut()
        .map(|co| connection_loop(co).fuse())
        .collect();

    let (new_connections_tx, new_connections_rx) = mpsc::unbounded();
    {
        let mut s_tx = TX.lock().unwrap();
        *s_tx = Some(new_connections_tx);
    }

    let new_loops = gen_new_loops(new_connections_rx).fuse();

    join!(join_all(loops), new_loops);
}

pub async fn gen_new_loops(rx: UnboundedReceiver<Connection>) {
    rx.for_each_concurrent(None, |mut co| async move {
        kill(&co.uuid).await;
        connection_loop(&mut co).await;
    })
    .await;
}

async fn connection_loop(co: &mut Connection) {
    if co.forbidden {
        log::info!("Ignoring connection for {}", &co.uuid);
        return;
    }
    log::info!("Starting connection for {}", &co.uuid);
    let mut socket = match SignalWebSocket::new(
        CONFIG.get_ws_endpoint(&co.uuid, co.device_id, &co.password),
        co.endpoint.clone(),
        co.strategy.clone(),
    ) {
        Ok(s) => s,
        Err(e) => {
            log::info!("An error occured for {}: {}", co.uuid, e);
            return;
        }
    };
    let metrics_future = set_metrics(&mut socket, co.strategy.clone());
    // Add the channel to kill the connection if needed
    let (kill_tx, mut kill_rx) = mpsc::unbounded();
    {
        REFS.lock().unwrap().push(LoopRef {
            uuid: co.uuid.clone(),
            tx: kill_tx,
        });
    }
    METRICS.connections.inc();
    // loop
    select!(
        res = socket.connection_loop().fuse() => handle_connection_closed(res, co),
        _ = kill_rx.next().fuse() => log::info!("Connection killed"),
        _ = metrics_future.fuse() => log::warn!("One of the metrics channel has been closed."),
    );
    // Remove the channel to kill the connection
    let mut refs = REFS.lock().unwrap();
    if let Some(i_ref) = refs.iter().position(|l_ref| l_ref.uuid.eq(&co.uuid)) {
        refs.remove(i_ref);
    }
    METRICS.connections.dec();
}

fn set_metrics(socket: &mut SignalWebSocket, strategy: Strategy) -> impl Future<Output = ()> {
    let strategy_type = strategy.to_string();
    let (on_message_tx, on_message_rx) = mpsc::unbounded::<u32>();
    let (on_push_tx, on_push_rx) = mpsc::unbounded::<u32>();
    let (on_reconnection_tx, on_reconnection_rx) = mpsc::unbounded::<u32>();
    socket.channels.on_message_tx = Some(on_message_tx);
    socket.channels.on_push_tx = Some(on_push_tx);
    socket.channels.on_reconnection_tx = Some(on_reconnection_tx);
    async move {
        select!(
            _ = on_message_rx
                .for_each(|_| async {
                    METRICS.messages.with_label_values(&[&strategy_type]).inc();
                })
                .fuse() => (),
            _ = on_push_rx
                .for_each(|_| async {
                    METRICS.pushs.with_label_values(&[&strategy_type]).inc();
                })
                .fuse() => (),
            _ = on_reconnection_rx
                .for_each(|_| async {
                    METRICS.reconnections.inc();
                })
                .fuse() => (),
        )
    }
}

fn handle_connection_closed(res: Result<()>, co: &mut Connection) {
    log::debug!("Connection closed.");

    match res {
        Ok(()) => (),
        Err(error) => {
            if let Some(http_error) = error.downcast_ref::<tungstenite::Error>() {
                if let tungstenite::Error::Http(resp) = http_error {
                    let status = resp.status();
                    log::info!("Connection for {} closed with status: {}", &co.uuid, status);
                    if status == 403 {
                        co.forbidden = true;
                        let _ = DB.add(co);
                    }
                }
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
