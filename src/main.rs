use lazy_static::lazy_static;

use config::Config;

mod cli;
mod config;
mod db;
mod signalwebsocket;
mod web;

lazy_static! {
    #[derive(Debug)]
    static ref CONFIG: Config = Config::load();
}

#[tokio::main]
async fn main() {
    env_logger::init();
    // dbg!(&*CONFIG);
    cli::cli().await;
}
