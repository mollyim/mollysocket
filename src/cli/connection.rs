use crate::{
    db::{self, OptTime},
    CONFIG,
};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ConnectionCommand {
    /// Add a new account connection
    Add {
        /// Account UUID
        account_id: String,

        /// Device number
        #[arg(value_parser = clap::value_parser!(u32).range(1..))]
        device_id: u32,

        /// Device token
        password: String,

        /// UnifiedPush endpoint
        endpoint: String,
    },

    /// List all account connections
    List {},

    /// Remove account connection
    Remove {
        /// Account UUID
        account_id: String,
    },
}

pub async fn connection(command: &ConnectionCommand) {
    match command {
        ConnectionCommand::Add{account_id, device_id, password, endpoint} => add(account_id, device_id, password, endpoint).await,
        ConnectionCommand::List{} => list(),
        ConnectionCommand::Remove{account_id} => rm(account_id),
    }

}

async fn add(uuid: &str, device_id: &u32, password: &str, endpoint: &str) {
    if !CONFIG.is_uuid_valid(uuid) {
        println!("UUID invalid or forbidden: {}", uuid);
        return
    }
    if !CONFIG.is_endpoint_valid(endpoint).await {
        println!("Endpoint invalid or forbidden: {}", endpoint);
        return
    }
    let _ = db::MollySocketDb::new().unwrap().add(&db::Connection {
        uuid: uuid.to_string(),
        device_id: *device_id,
        password: password.to_string(),
        endpoint: endpoint.to_string(),
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

fn rm(uuid: &str) {
    db::MollySocketDb::new().unwrap().rm(uuid).unwrap();
    println!("Connection for {} successfully removed.", uuid)
}