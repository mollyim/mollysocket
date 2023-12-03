use crate::{config, db::MollySocketDb};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum TestCommand {
    /// Test allowed UnifiedPush endpoint
    Endpoint {
        /// UnifiedPush endpoint
        endpoint: String,
    },

    /// Test allowed account uuid
    Uuid {
        /// Account uuid
        account_id: String,
    },
}

pub async fn test(command: &TestCommand) {
    config::print();
    match command {
        TestCommand::Endpoint { endpoint } => test_endpoint(endpoint).await,
        TestCommand::Uuid { account_id } => test_uuid(account_id),
    }
}

fn test_uuid(uuid: &str) {
    if !config::is_uuid_valid(uuid) {
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
    if config::is_endpoint_valid(endpoint).await {
        println!("Endpoint {} is valid", endpoint);
    } else {
        println!("Endpoint {} is not valid", endpoint);
    }
}
