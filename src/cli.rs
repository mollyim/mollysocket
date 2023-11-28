use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::cli::{connection::ConnectionCommand, test::TestCommand};

mod connection;
mod server;
mod test;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(infer_subcommands = true)]
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
    Server {},

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
}

pub async fn cli() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Server {} => server::server().await,
        Command::Connection { command } => connection::connection(command).await,
        Command::Test { command } => test::test(command).await,
    }
}
