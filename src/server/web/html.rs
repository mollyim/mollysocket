use crate::{qrcode, vapid};

macro_rules! index {
    ($v:expr) => {
        format!(
            r#"
<!DOCTYPE html>
<html>
<head>
<title>MollySocket</title>
</head>
<body>
<h1>MollySocket</h1>
{}
<p>Version {}</p>
</body>
</html>
        "#,
            $v,
            env!("CARGO_PKG_VERSION")
        )
    };
}

pub fn get_index() -> String {
    let intro = qrcode::INTRO;
    let url = match qrcode::gen_url() {
        Ok(u) => u,
        Err(e) => {
            if let Some(vapid::Error::VapidKeyError) = e.downcast_ref::<vapid::Error>() {
                return no_vapid();
            } else if let Some(qrcode::Error::NoUrlDefinedError) = e.downcast_ref::<qrcode::Error>()
            {
                return no_url();
            }
            return generic_error();
        }
    };
    let qr = qrcode::url_to_svg_qr(&url);

    index!(format!(
        r#"
<p>{intro}<br><a href="{url}">{url}</a></p>
<div style="max-width: 25rem;">
{qr}
</div>
        "#,
    ))
}

fn no_vapid() -> String {
    index!("<p>VAPID Key not found. Configure a VAPID key and try again.</p>")
}

fn no_url() -> String {
    index!("<p>URL is not defined. Configure the URL and try again.</p>")
}

fn generic_error() -> String {
    index!("<p>An error occurred. You should check the server logs.</p>")
}
