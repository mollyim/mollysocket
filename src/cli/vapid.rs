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
    let header = vapid::gen_vapid_header(origin)
        .expect("Couldn't generate header. Have you configured the VAPID key ?");
    println!("{}", header);
}
