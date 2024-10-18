use crate::qrcode;

/// Print mollysocket link URL and show the associated QR Code
pub fn qrcode() {
    let url = qrcode::gen_url().unwrap();
    let qr_code = qrcode::url_to_printable_qr(&url);
    println!("{}\n{}\n{}", qrcode::INTRO, url.to_string(), qr_code)
}
