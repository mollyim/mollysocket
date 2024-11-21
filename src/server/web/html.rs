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
<script>
let ms_link = new URL(document.getElementById("ms-link").href)
let param_url
if (param_url = ms_link.searchParams.get("url")) {{
    let parsed_url = new URL(param_url)
    if (document.location.origin != parsed_url.origin) {{
        alert(`Origin doesn't seem to be correctly passed to mollysocket. Expecting ${{document.location.origin}}, found ${{parsed_url.origin}}

You may have forgotten to proxy pass the Host value to mollysocket, or wish to use this server in airgapped mode.`)
    }} else if (document.location.pathname != parsed_url.pathname) {{
        alert(`Pathname doesn't seem to be correctly passed to mollysocket. Expecting ${{document.location.pathname}}, found ${{parsed_url.pathname}}

You may have forgotten to proxy pass the path value to mollysocket (with X-Original-URL header), or wish to use this server in airgapped mode.`)
    }}
}}
</script>
</body>
</html>
        "#,
            $v,
            env!("CARGO_PKG_VERSION")
        )
    };
}

pub fn get_index(airgapped: bool, ms_url: Option<&str>) -> String {
    let intro = qrcode::INTRO;
    let url = if airgapped {
        qrcode::gen_url_airgapped()
    } else {
        let ms_url = match ms_url {
            Some(u) => u,
            None => return no_url(),
        };
        qrcode::gen_url(ms_url)
    };

    let url = match url {
        Ok(u) => u,
        Err(e) => {
            if let Some(vapid::Error::VapidKeyError) = e.downcast_ref::<vapid::Error>() {
                return no_vapid();
            }
            return generic_error();
        }
    };
    let qr = qrcode::url_to_svg_qr(&url);

    if airgapped {
        index!(format!(
            r#"
<p>⚠️<u>This will configure your server in air gapped mode</u>⚠️<br>
Molly won't be able to update push information if necessary.<br>
You can also keep a screenshot of this QR code in case you need to reconfigure your server without having access to it.<br><br>
<p>{intro}</p>
<a hidden id="ms-link" href="{url}">{url}</a>
<div style="max-width: 25rem;">
<span hidden id="ms_link" link="{url}" /></span>
{qr}
</div>
<p><i>Wish to use <a href="?">with the webserver</a> ?</i></p>
        "#,
        ))
    } else {
        index!(format!(
            r#"
<p>{intro}</p>
<a hidden id="ms-link" href="{url}">{url}</a>
<div style="max-width: 25rem;">
{qr}
</div>
<p><i>Wish to use in <a href="?airgapped">airgapped mode</a> ?</i></p>
        "#,
        ))
    }
}

fn no_vapid() -> String {
    index!("<p>VAPID Key not found. <a href=\"https://github.com/mollyim/mollysocket?tab=readme-ov-file#vapid-key\">Configure a VAPID key and try again.</a></p>")
}

fn no_url() -> String {
    index!("<p>URL not found. The request seems to be incorrectly formatted.</p>")
}

fn generic_error() -> String {
    index!("<p>An error occurred. You should check the server logs.</p>")
}
