use lazy_static::lazy_static;

use config::Config;

mod cli;
mod config;
mod db;
mod error;
mod server;
mod signalwebsocket;

lazy_static! {
    #[derive(Debug)]
    static ref CONFIG: Config = Config::load(None);
}

#[tokio::main]
async fn main() {
    env_logger::init();
    // dbg!(&*CONFIG);
    cli::cli().await;
}
