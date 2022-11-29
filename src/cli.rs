use std::env;

mod connection;
mod oneshot;
mod server;

fn usage() {
    println!(
        "
Usage: {0} [command] [args, ...]

Commands:
  server       Run webserver and websockets
  connection    List, add and remove connections

Run '{0} [command] --help' for more information on a command.
",
        env::args().nth(0).unwrap()
    );
}

pub async fn cli() {
    let mut args = env::args();
    args.next();
    match args.next() {
        Some(cmd) if cmd == "oneshot" || cmd == "o" => oneshot::oneshot(args).await,
        Some(cmd) if cmd == "connection" || cmd == "c" => connection::connection(args).await,
        Some(cmd) if cmd == "server" || cmd == "s" => server::server(args).await,
        _ => usage(),
    }
}
