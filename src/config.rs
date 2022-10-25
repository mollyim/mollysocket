use std::{default::Default, fmt::Debug};
use user_config::{Environment, UserConfig};

mod user_config;

#[derive(Debug)]
pub struct Config {
    pub version: String,
    pub ws_endpoint: String,
    pub allowed_endpoints: Vec<url::Url>,
    pub allowed_uuids: Vec<String>,
}

impl Config {
    pub fn load() -> Config {
        let user_cfg = UserConfig::load().unwrap_or_else(|_| UserConfig::default());
        Config {
            version: String::from(option_env!("CARGO_PKG_VERSION").unwrap_or_else(|| "Unknown")),
            ws_endpoint: String::from(match user_cfg.environment {
                Environment::PROD => "wss://chat.signal.org/v1/websocket/?login=%s.%s&password=%s",
                Environment::STAGING => {
                    "wss://chat.staging.signal.org/v1/websocket/?login=%s.%s&password=%s"
                }
            }),
            allowed_endpoints: user_cfg
                .allowed_endpoints
                .into_iter()
                .map(|endpoint| url::Url::parse(&endpoint).unwrap())
                .collect(),
            allowed_uuids: user_cfg.allowed_uuids,
        }
    }

    pub fn is_uuid_valid(&self, uuid: &str) -> bool {
        self.allowed_uuids
            .iter()
            .any(|allowed| allowed == "*" || allowed == uuid)
    }

    pub fn is_endpoint_valid(&self, url: &url::Url) -> bool {
        self.allowed_endpoints.iter().any(|allowed| {
            url.host() == allowed.host()
                && url.port() == allowed.port()
                && url.scheme() == allowed.scheme()
                && url.username() == allowed.username()
                && url.password() == allowed.password()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config(uuid: &str) -> Config {
        Config {
            version: String::from("tests"),
            ws_endpoint: String::from("tests"),
            allowed_endpoints: vec![url::Url::parse("http://0.0.0.0/").unwrap()],
            allowed_uuids: vec![String::from(uuid)],
        }
    }

    #[test]
    fn check_wildcard_uuid() {
        let cfg = test_config("*");
        assert!(cfg.is_uuid_valid("0d2ff653-3d88-43de-bcdb-f6657d3484e4"));
    }

    #[test]
    fn check_defined_uuid() {
        let cfg = test_config("0d2ff653-3d88-43de-bcdb-f6657d3484e4");
        assert!(cfg.is_uuid_valid("0d2ff653-3d88-43de-bcdb-f6657d3484e4"));
        assert!(!cfg.is_uuid_valid("11111111-3d88-43de-bcdb-f6657d3484e4"));
    }

    #[test]
    fn check_endpoint() {
        let cfg = test_config("*");
        assert!(cfg.is_endpoint_valid(&url::Url::parse("http://0.0.0.0/foo?blah").unwrap()));
        assert!(!cfg.is_endpoint_valid(&url::Url::parse("http://0.0.0.0:8080/foo?blah").unwrap()));
        assert!(
            !cfg.is_endpoint_valid(&url::Url::parse("http://user:pass@0.0.0.0/foo?blah").unwrap())
        );
        assert!(!cfg.is_endpoint_valid(&url::Url::parse("https://0.0.0.0/foo?blah").unwrap()));
    }
}
