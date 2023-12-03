mod cli;
mod config;
mod db;
mod server;
mod utils;
mod ws;

#[tokio::main]
async fn main() {
    cli::cli().await;
}
