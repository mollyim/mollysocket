use crate::db::MollySocketDb;
use futures_util::{join, FutureExt};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

mod connections;
mod web;

lazy_static! {
    static ref REFS: Arc<Mutex<Vec<connections::LoopRef>>> = Arc::new(Mutex::new(vec![]));
    static ref DB: MollySocketDb = MollySocketDb::new().unwrap();
    static ref TX: Arc<Mutex<connections::OptSender>> = Arc::new(Mutex::new(None));
}

pub async fn run() {
    join!(web::launch().fuse(), connections::run().fuse());
}
