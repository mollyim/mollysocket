use lazy_static::lazy_static;
use std::env;

use config::Config;

mod cli;
mod config;
mod signalwebsocket;

lazy_static! {
    #[derive(Debug)]
    static ref CONFIG: Config = Config::load();
}

fn usage() -> Result<(), ()> {
    println!(
        "
Usage: {0} [command] [args, ...]

Commands:
  oneshot      Connect to a websocket and push to the endpoint
  server       Run webserver and websockets
  endpoints    List, add and remove endpoints

Run '{0} [command] --help' for more information on a command.
",
        env::args().nth(0).unwrap()
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init();
    // dbg!(&*CONFIG);
    let mut args = env::args();
    args.next();
    match args.next() {
        Some(cmd) if cmd == "oneshot" || cmd == "o" => cli::oneshot::oneshot(args).await,
        _ => usage(),
    }
}
