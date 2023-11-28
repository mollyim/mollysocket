use crate::{db::MollySocketDb, CONFIG};
use clap::Subcommand;
use std::env;

#[derive(Subcommand)]
pub enum TestCommand {
    /// Test allowed UnifiedPush endpoint
    Endpoint {
        /// Unified push endpoint
        endpoint: String,
    },

    /// Test allowed account uuid
    Uuid {
        /// Account uuid
        account_id: String,
    },
}

pub async fn test(command: &TestCommand) {
    match command {
        TestCommand::Endpoint{endpoint} => test_endpoint(&endpoint).await,
        TestCommand::Uuid{account_id} => test_uuid(&account_id),
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
