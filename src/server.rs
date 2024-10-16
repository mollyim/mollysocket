use crate::{db::MollySocketDb, server::metrics::Metrics};
use futures_util::{future::join, pin_mut, select, FutureExt};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tokio::signal;

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
    let signal_future = signal::ctrl_c().fuse();
    let joined_future = join(web::launch().fuse(), connections::run().fuse());

    pin_mut!(signal_future, joined_future);

    select!(
        _ = signal_future => log::info!("SIGINT received"),
        _ = joined_future => log::warn!("Server stopped"),
    )
}
