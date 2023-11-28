use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::cli::{
    test::TestCommand,
    connection::ConnectionCommand
};

mod connection;
mod oneshot;
mod server;
mod test;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, default_value_t = false)]
    debug: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run webserver and websockets
    Server{},

    /// Add, remove and list connections
    Connection {
        #[command(subcommand)]
        command: ConnectionCommand,
    },

    /// Test account and endpoint validity
    Test {
        #[command(subcommand)]
        command: TestCommand,
    },

    /// Oneshot
    Oneshot {
        /// Signal websocket address
        connect_addr: String,
        /// Unified push endpoint
        push_endpoint: String
    },
}

pub async fn cli() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Server{} => server::server().await,
        Command::Connection { command } => connection::connection(command).await,
        Command::Test { command } => test::test(&command).await,
        Command::Oneshot { connect_addr, push_endpoint } => oneshot::oneshot(connect_addr, push_endpoint).await,
    }
}
