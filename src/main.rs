use std::sync::OnceLock;

mod cli;
mod config;
mod db;
mod server;
mod utils;
mod ws;

static CONFIG: OnceLock<config::Config> = OnceLock::new();

#[tokio::main]
async fn main() {
    env_logger::init();
    cli::cli().await;
}
