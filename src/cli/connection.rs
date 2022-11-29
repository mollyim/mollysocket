use crate::{
    db::{self, OptTime, Strategy},
    CONFIG,
};
use std::{
    env::{self, Args},
    str::FromStr,
};

fn usage() {
    println!(
        "
Usage: {} connection [command] [args, ...]

Commands:            
  add [uuid] [device_id] [password] [endpoint] [strategy]
  list
  rm [uuid]

Strategies:
  rest        Send all messages
  websocket   Send all messages at least 5 seconds apart
",
        env::args().nth(0).unwrap()
    );
}

pub async fn connection(args: Args) {
    let argv: Vec<String> = args.collect();
    if argv.iter().any(|arg| arg == "--help" || arg == "-h") {
        return usage();
    };
    match argv.get(0) {
        Some(cmd) if cmd == "add" || cmd == "a" => add(argv).await,
        Some(cmd) if cmd == "rm" || cmd == "r" => rm(argv),
        Some(cmd) if cmd == "list" || cmd == "l" => list(),
        _ => return usage(),
    };
}

async fn add(mut argv: Vec<String>) {
    argv.remove(0);
    let uuid = match argv.get(0) {
        Some(argv1) => {
            if CONFIG.is_uuid_valid(argv1) {
                argv1
            } else {
                println!("UUID invalid or forbidden: {}", argv1);
                return usage();
            }
        }
        _ => {
            return usage();
        }
    }
    .clone();
    let device_id = match argv.get(1) {
        Some(argv2) => {
            if is_valid_int(argv2) {
                argv2.parse::<u32>().unwrap()
            } else {
                println!("Device_id invalid: {}", argv2);
                return usage();
            }
        }
        _ => {
            return usage();
        }
    }
    .clone();
    let password = match argv.get(2) {
        Some(argv3) => argv3,
        _ => {
            return usage();
        }
    }
    .clone();
    let endpoint = match argv.get(3) {
        Some(argv4) => {
            if CONFIG.is_endpoint_valid(argv4).await {
                argv4
            } else {
                println!("Endpoint invalid or forbidden: {}", argv4);
                return usage();
            }
        }
        _ => {
            return usage();
        }
    }
    .clone();
    let strategy = match argv.get(4) {
        Some(argv5) => match Strategy::from_str(argv5) {
            Ok(s) => s,
            Err(_) => {
                println!("Invalid strategy: {}", argv5);
                usage();
                return;
            }
        },
        None => {
            usage();
            return;
        }
    }
    .clone();

    let _ = db::MollySocketDb::new().unwrap().add(&db::Connection {
        uuid: uuid.clone(),
        device_id,
        password,
        endpoint,
        strategy,
        forbidden: false,
        last_registration: OptTime(None),
    });
    println!("Connection for {} added.", uuid);
}

fn list() {
    db::MollySocketDb::new()
        .unwrap()
        .list()
        .unwrap()
        .iter()
        .for_each(|connection| {
            dbg!(&connection);
        });
}

fn rm(mut argv: Vec<String>) {
    argv.remove(0);
    let uuid = match argv.get(0) {
        Some(argv1) => {
            if CONFIG.is_uuid_valid(argv1) {
                argv1
            } else {
                println!("UUID invalid or forbidden: {}", argv1);
                return usage();
            }
        }
        _ => {
            return usage();
        }
    };
    db::MollySocketDb::new().unwrap().rm(uuid).unwrap();
    println!("Connection for {} successfully removed.", uuid)
}

fn is_valid_int(value: &str) -> bool {
    match value.parse::<u32>() {
        Ok(_) => true,
        Err(_) => false,
    }
}
