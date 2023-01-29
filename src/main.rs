#![feature(ip)]
use lazy_static::lazy_static;

use config::Config;

mod cli;
mod config;
mod db;
mod server;
mod utils;
mod ws;

lazy_static! {
    #[derive(Debug)]
    static ref CONFIG: Config = Config::load(None);
}

#[tokio::main]
async fn main() {
    env_logger::init();
    cli::cli().await;
}
