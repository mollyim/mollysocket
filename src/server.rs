use crate::db::MollySocketDb;
use futures_util::{future::join, pin_mut, select, FutureExt};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tokio::signal;

mod connections;
mod web;

lazy_static! {
    static ref REFS: Arc<Mutex<Vec<connections::LoopRef>>> = Arc::new(Mutex::new(vec![]));
    static ref DB: MollySocketDb = MollySocketDb::new().unwrap();
    static ref TX: Arc<Mutex<connections::OptSender>> = Arc::new(Mutex::new(None));
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
