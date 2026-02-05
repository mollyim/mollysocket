use eyre::Result;
use rocket::serde::json::json;
use url::Url;

pub mod post_allowed;

pub fn anonymize_url(url_in: &str) -> String {
    let mut mut_url = url::Url::parse(url_in).unwrap();
    mut_url.set_host(Some("fake.domain.tld")).unwrap();
    let path = format!("{}...", &mut_url.path()[..5]);
    mut_url.set_path(&path);
    mut_url.path();
    mut_url.into()
}

pub async fn ping(url: Url) -> Result<reqwest::Response> {
    let res = post_allowed::post_allowed(url, &json!({"test":true}), Some("test")).await?;
    res.error_for_status_ref()?;
    Ok(res)
}
