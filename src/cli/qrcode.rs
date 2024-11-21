use crate::{qrcode, vapid};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum QrcodeCommand {
    /// Generate link QR code for the associated URL
    Url {
        /// URL of mollysocket
        url: String,
    },

    /// Generate link QR code for mollysocket used in airgapped mode
    Airgapped {},
}

/// Print mollysocket link URL and show the associated QR Code
pub fn qrcode(command: &QrcodeCommand) {
    let url = match command {
        QrcodeCommand::Url { url } => qrcode::gen_url(url),
        QrcodeCommand::Airgapped {} => qrcode::gen_url_airgapped(),
    };
    if let Err(e) = &url {
        if let Some(vapid::Error::VapidKeyError) = e.downcast_ref::<vapid::Error>() {
            println!("{}", e);
            return;
        }
    }
    let url = url.unwrap();
    let qr_code = qrcode::url_to_printable_qr(&url);
    println!("{}\n{}\n{}", qrcode::INTRO, url.to_string(), qr_code)
}
