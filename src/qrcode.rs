use eyre::Result;
use qrcodegen::{QrCode, QrCodeEcc};
use url::Url;

use crate::vapid;

pub const INTRO: &str =
    "Scan the folowing QR code to link mollysocket, or enter the following link manually:";

/// Generate deep link to link mollysocket to molly with url
pub fn gen_url(ms_url: &str) -> Result<Url> {
    let mut url = Url::parse("mollysocket://link")?;
    let vapid = vapid::get_vapid_pubkey()?;
    url.query_pairs_mut().append_pair("vapid", vapid);
    url.query_pairs_mut().append_pair("url", ms_url);
    url.query_pairs_mut().append_pair("type", "webserver");
    Ok(url)
}

/// Generate deep link to link mollysocket to molly in airgapped mode
pub fn gen_url_airgapped() -> Result<Url> {
    let mut url = Url::parse("mollysocket://link")?;
    let vapid = vapid::get_vapid_pubkey()?;
    url.query_pairs_mut().append_pair("vapid", vapid);
    url.query_pairs_mut().append_pair("type", "airgapped");
    Ok(url)
}

/// Return QRCode made with characters
pub fn url_to_printable_qr(url: &Url) -> String {
    let qr = QrCode::encode_text(&url.as_str(), QrCodeEcc::Low).unwrap();
    let mut result = String::new();
    let border: i32 = 4;
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

/// Return QRCode in svg format
pub fn url_to_svg_qr(url: &Url) -> String {
    let qr = QrCode::encode_text(&url.as_str(), QrCodeEcc::Low).unwrap();
    let mut result = String::new();
    let border: i32 = 4;
    result += "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
    result += "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n";
    let dimension = qr
        .size()
        .checked_add(border.checked_mul(2).unwrap())
        .unwrap();
    result += &format!(
		"<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 {0} {0}\" stroke=\"none\">\n", dimension);
    result += "\t<rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>\n";
    result += "\t<path d=\"";
    for y in 0..qr.size() {
        for x in 0..qr.size() {
            if qr.get_module(x, y) {
                if x != 0 || y != 0 {
                    result += " ";
                }
                result += &format!("M{},{}h1v1h-1z", x + border, y + border);
            }
        }
    }
    result += "\" fill=\"#000000\"/>\n";
    result += "</svg>\n";
    result
}
