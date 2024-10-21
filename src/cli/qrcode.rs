use crate::qrcode;
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
    }
    .unwrap();
    let qr_code = qrcode::url_to_printable_qr(&url);
    println!("{}\n{}\n{}", qrcode::INTRO, url.to_string(), qr_code)
}
