use lazy_static::lazy_static;
use std::env;

use config::Config;

mod cli;
mod config;
mod db;
mod signalwebsocket;

lazy_static! {
    #[derive(Debug)]
    static ref CONFIG: Config = Config::load();
}

fn usage() {
    println!(
        "
Usage: {0} [command] [args, ...]

Commands:
  oneshot      Connect to a websocket and push to the endpoint
  server       Run webserver and websockets
  connection    List, add and remove connections

Run '{0} [command] --help' for more information on a command.
",
        env::args().nth(0).unwrap()
    );
}

#[tokio::main]
async fn main() {
    env_logger::init();
    // dbg!(&*CONFIG);
    let mut args = env::args();
    args.next();
    match args.next() {
        Some(cmd) if cmd == "oneshot" || cmd == "o" => cli::oneshot::oneshot(args).await,
        Some(cmd) if cmd == "connection" || cmd == "c" => cli::connection::connection(args),
        _ => usage(),
    }
}
