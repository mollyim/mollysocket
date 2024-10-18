mod cli;
mod config;
mod db;
mod qrcode;
mod server;
mod utils;
mod vapid;
mod ws;

#[tokio::main]
async fn main() {
    cli::cli().await;
}
