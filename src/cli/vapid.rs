use crate::vapid;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum VapidCommand {
    /// Try to generate a VAPID header for endpoint
    Test {
        /// UnifiedPush endpoint
        endpoint: String,
    },

    /// Generate VAPID key and print to STDOUT
    Generate {},
}

pub fn vapid(command: &VapidCommand) {
    match command {
        VapidCommand::Test { endpoint } => print_vapid_for_endpoint(endpoint),
        VapidCommand::Generate {} => generate_vapid(),
    }
}

fn generate_vapid() {
    let key = vapid::gen_vapid_key();
    println!("{}", key);
}

fn print_vapid_for_endpoint(endpoint: &str) {
    let origin = url::Url::parse(endpoint)
        .expect(&format!("Could not parse {}.", endpoint))
        .origin();
    let header = match vapid::get_vapid_header(origin) {
        Err(e) if matches!(e.downcast_ref(), Some(vapid::Error::VapidKeyError)) => {
            println!("{}", e);
            return;
        }
        h => h,
    }
    .expect("Cannot generate header");
    println!("{}", header);
}
