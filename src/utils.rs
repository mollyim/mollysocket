pub mod post_allowed;

pub fn anonymize_url(url_in: &str) -> String {
    let mut mut_url = url::Url::parse(url_in).unwrap();
    mut_url.set_host(Some("fake.domain.tld")).unwrap();
    mut_url.into()
}
