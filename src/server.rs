use crate::{db::MollySocketDb, server::metrics::Metrics};
use futures_util::{future::join, pin_mut, select, FutureExt};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tokio::signal;
#[cfg(unix)]
use tokio::signal::unix::{self, SignalKind};

mod connections;
mod metrics;
mod web;

lazy_static! {
    static ref DB: MollySocketDb = MollySocketDb::new().unwrap();
    static ref METRICS: Metrics = Metrics::new().unwrap();
    /**
    Vec of [connections::KillLoopRef].

    Filled by [connections].

    When a message is sent to the kill channel associated to the uuid, the loop for the registration stops.
    */
    static ref KILL_VEC: Arc<Mutex<Vec<connections::KillLoopRef>>> = Arc::new(Mutex::new(vec![]));
    /**
     Channel to do action when a new connection is registered.

     Bounded by [connections].

     When a new connection is sent, loops for connection with this [Connection][crate::db::Connection]#uuid is kill, and a new loop is started.
     */
    static ref NEW_CO_TX: Arc<Mutex<connections::OptSender>> = Arc::new(Mutex::new(None));
}

pub async fn run() {
    let sigint_future = signal::ctrl_c().fuse();
    #[cfg(unix)]
    let mut sigterm_stream = unix::signal(SignalKind::terminate()).unwrap();
    #[cfg(unix)]
    let sigterm_future = sigterm_stream.recv().fuse();
    #[cfg(not(unix))]
    let sigterm_future = std::future::pending().fuse();
    let joined_future = join(web::launch().fuse(), connections::run().fuse());

    pin_mut!(sigint_future, sigterm_future, joined_future);

    select!(
        _ = sigint_future => log::info!("SIGINT received"),
        _ = sigterm_future => log::info!("SIGTERM received"),
        _ = joined_future => log::warn!("Server stopped"),
    )
}
