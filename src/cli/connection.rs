use crate::{
    config,
    db::{self, OptTime},
    utils::anonymize_url,
};
use clap::Subcommand;
use lazy_static::lazy_static;
use regex::Regex;

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
    List {
        /// Anonymize account id and password
        #[arg(short, long)]
        anonymized: bool,
    },

    /// Remove account connection
    Remove {
        /// Account UUID
        account_id: String,
    },
}

pub async fn connection(command: &ConnectionCommand) {
    match command {
        ConnectionCommand::Add {
            account_id,
            device_id,
            password,
            endpoint,
        } => add(account_id, device_id, password, endpoint).await,
        ConnectionCommand::List { anonymized } => list(*anonymized),
        ConnectionCommand::Remove { account_id } => rm(account_id),
    }
}

async fn add(uuid: &str, device_id: &u32, password: &str, endpoint: &str) {
    if !config::is_uuid_valid(uuid) {
        println!("UUID invalid or forbidden: {}", uuid);
        return;
    }
    if !config::is_endpoint_valid(endpoint).await {
        println!("Endpoint invalid or forbidden: {}", endpoint);
        return;
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

fn list(anonymized: bool) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[^-]").unwrap();
    }
    if anonymized {
        println!("
/!\\ The endpoints are not fully anonymized. /!\\
This is required to help to debug some setups. You should unregister Molly from your distributor to get a new endpoint if you share this output.
");
    }
    db::MollySocketDb::new()
        .unwrap()
        .list()
        .unwrap()
        .iter_mut()
        .for_each(|connection| {
            if anonymized {
                connection.uuid = RE.replace_all(&connection.uuid, "x").into();
                connection.password = RE.replace_all(&connection.password, "x").into();
                connection.endpoint = anonymize_url(&connection.endpoint);
            }
            dbg!(&connection);
        });
}

fn rm(uuid: &str) {
    db::MollySocketDb::new().unwrap().rm(uuid).unwrap();
    println!("Connection for {} successfully removed.", uuid)
}
