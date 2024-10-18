use eyre::{eyre, Result};
use qrcodegen::{QrCode, QrCodeEcc};
use url::Url;

use crate::{config, vapid};

pub const INTRO: &str =
    "Scan the folowing QR code to link mollysocket, or enter the following link manually:";

/// Generate deep link to link mollysocket to molly
pub fn gen_url() -> Result<Url> {
    let mut url = Url::parse("mollysocket://link")?;
    let vapid = vapid::get_vapid_pubkey()?;
    url.query_pairs_mut().append_pair("vapid", vapid);
    if config::should_start_webserver() {
        let ms_url = config::get_url();
        if ms_url.is_empty() {
            return Err(eyre!("Webserver is enabled but URL is not defined."));
        }
        url.query_pairs_mut().append_pair("url", &ms_url);
        url.query_pairs_mut().append_pair("type", "webserver");
    } else {
        url.query_pairs_mut().append_pair("type", "airgapped");
    }
    Ok(url)
}

/// Return QRCode made with characters
pub fn url_to_printable_qr(url: &Url) -> String {
    let qr = QrCode::encode_text(&url.as_str(), QrCodeEcc::Low).unwrap();
    let mut result = String::new();
    let border: i32 = 2;
    for y in (-border..qr.size() + border).step_by(2) {
        for x in -border..qr.size() + border {
            let c: char = if qr.get_module(x, y) {
                if qr.get_module(x, y + 1) {
                    '█'
                } else {
                    '▀'
                }
            } else {
                if qr.get_module(x, y + 1) {
                    '▄'
                } else {
                    ' '
                }
            };
            result.push(c);
        }
        result.push('\n');
    }
    result.push('\n');
    result
}
