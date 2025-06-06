use crate::{
    db::Connection,
    server::{DB, KILL_VEC, METRICS, NEW_CO_TX},
    ws::SignalWebSocket,
};
use eyre::Result;
use futures_channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures_util::{future::join_all, join, select, Future, FutureExt, StreamExt};
use tokio_tungstenite::tungstenite;

/**
Associates the kill channel to the [Connection][crate::db::Connection]#uuid.
*/
pub struct KillLoopRef {
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
        let mut s_tx = NEW_CO_TX.lock().unwrap();
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
    loop {
        if co.forbidden {
            log::info!("Ignoring connection for {}", &co.uuid);
            METRICS.forbiddens.inc();
            return;
        }
        log::info!("Starting connection for {}", &co.uuid);
        let mut socket =
            match SignalWebSocket::new(&co.uuid, co.device_id, &co.password, &co.endpoint) {
                Ok(s) => s,
                Err(e) => {
                    log::info!("An error occured for {}: {}", co.uuid, e);
                    return;
                }
            };
        let metrics_future = set_metrics(&mut socket);
        // Add the channel to kill the connection if needed
        let (kill_tx, mut kill_rx) = mpsc::unbounded();
        {
            KILL_VEC.lock().unwrap().push(KillLoopRef {
                uuid: co.uuid.clone(),
                tx: kill_tx,
            });
        }
        METRICS.connections.inc();
        // bool to stop looping if the connection has been explicitely killed.
        let mut stop_loop = false;
        // loop connection
        select!(
            res = socket.connection_loop().fuse() => handle_connection_closed(res, co),
            _ = metrics_future.fuse() => log::warn!("[{}] One of the metrics channel has been closed.", co.uuid),
            _ = kill_rx.next().fuse() => {
                log::info!("[{}] Connection killed", co.uuid);
                // We don't want the loop to restart if the connection has been killed.
                stop_loop = true;
                },
        );
        // Remove the channel to kill the connection
        let mut refs = KILL_VEC.lock().unwrap();
        if let Some(i_ref) = refs.iter().position(|l_ref| l_ref.uuid.eq(&co.uuid)) {
            refs.remove(i_ref);
        }
        METRICS.connections.dec();
        // the connection has been killed, we don't loop.
        if stop_loop {
            return;
        }
    }
}

fn set_metrics(socket: &mut SignalWebSocket) -> impl Future<Output = ()> {
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
                    METRICS.messages.inc();
                })
                .fuse() => (),
            _ = on_push_rx
                .for_each(|_| async {
                    METRICS.pushs.inc();
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
            if let Some(tungstenite::Error::Http(resp)) = error.downcast_ref::<tungstenite::Error>()
            {
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

async fn kill(uuid: &str) {
    let refs = KILL_VEC.lock().unwrap();
    if let Some(l_ref) = refs.iter().find(|&l_ref| l_ref.uuid.eq(uuid)) {
        let _ = l_ref.tx.clone().unbounded_send(true);
    }
}
