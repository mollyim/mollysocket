use crate::{db::MollySocketDb, CONFIG};
use std::env::{self, Args};

fn usage() {
    println!(
        "
Usage:
{} test endpoint https://push.server.ltd/id
{} test uuid aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa
",
        env::args().nth(0).unwrap(),
        env::args().nth(0).unwrap(),
    );
}

pub async fn test(args: Args) {
    let argv: Vec<String> = args.collect();
    if argv.iter().any(|arg| arg == "--help" || arg == "-h") {
        usage();
        return;
    }
    let arg = match argv.get(1) {
        Some(arg) => arg,
        None => {
            usage();
            return;
        }
    }
    .clone();
    match argv.get(0) {
        Some(cmd) if cmd == "endpoint" || cmd == "e" => test_endpoint(&arg).await,
        Some(cmd) if cmd == "uuid" || cmd == "u" => test_uuid(&arg),
        _ => {
            usage();
            return;
        }
    }
}

fn print_cfg() {
    let file = match env::var_os("MOLLY_CONF") {
        Some(path) => path.into_string().unwrap_or("Error".to_string()),
        None => "Default".to_string(),
    };
    println!("Config file: {}", file);
}
fn test_uuid(uuid: &str) {
    print_cfg();
    if !CONFIG.is_uuid_valid(uuid) {
        println!("UUID {} is not valid", uuid);
    } else {
        println!("UUID {} is valid", uuid);
    }

    let db = match MollySocketDb::new() {
        Ok(db) => db,
        Err(_) => {
            println!("  An error occured while opening the DB.");
            return;
        }
    };
    let co = match db.get(uuid) {
        Ok(co) => co,
        Err(_) => {
            println!("  No connection is registered with this UUID.");
            return;
        }
    };
    if co.forbidden {
        println!("  The connection associated to this UUID is forbidden.");
        return;
    }
    println!("  A connection is associated to this UUID and is ok.");
}

async fn test_endpoint(endpoint: &str) {
    print_cfg();
    if CONFIG.is_endpoint_valid(endpoint).await {
        println!("Endpoint {} is valid", endpoint);
    } else {
        println!("Endpoint {} is not valid", endpoint);
        if CONFIG
            .user_cfg
            .allowed_endpoints
            .contains(&String::from("*"))
        {
            println!("  The endpoint does not resolve to a global IP.")
        }
        println!("  Below the allowed endpoints:");
        CONFIG
            .user_cfg
            .allowed_endpoints
            .iter()
            .for_each(|endpoint| {
                println!("    '{}'", endpoint);
            })
    }
}
