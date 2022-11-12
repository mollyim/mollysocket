use std::{default::Default, fmt::Debug};
use user_config::{Environment, UserConfig};

mod user_config;

#[derive(Debug)]
pub struct Config {
    pub version: String,
    pub user_cfg: UserConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config::load()
    }
}

impl Config {
    pub fn load() -> Config {
        let user_cfg = UserConfig::load().unwrap_or_else(|_| UserConfig::default());
        Config {
            version: String::from(option_env!("CARGO_PKG_VERSION").unwrap_or_else(|| "Unknown")),
            user_cfg,
        }
    }

    pub fn is_uuid_valid(&self, uuid: &str) -> bool {
        self.user_cfg
            .allowed_uuids
            .iter()
            .any(|allowed| allowed == "*" || allowed == uuid)
    }

    pub fn is_endpoint_valid(&self, url: &str) -> bool {
        if let Ok(url) = url::Url::parse(url) {
            return self.is_url_endpoint_valid(&url);
        }
        false
    }

    pub fn is_url_endpoint_valid(&self, url: &url::Url) -> bool {
        self.user_cfg.allowed_endpoints.iter().any(|allowed| {
            if let Ok(allowed_url) = url::Url::parse(allowed) {
                return url.host() == allowed_url.host()
                    && url.port() == allowed_url.port()
                    && url.scheme() == allowed_url.scheme()
                    && url.username() == allowed_url.username()
                    && url.password() == allowed_url.password();
            }
            false
        })
    }

    pub fn get_ws_endpoint(&self, uuid: &str, devide_id: u32, password: &str) -> String {
        match self.user_cfg.environment {
            Environment::PROD => format!(
                "wss://chat.signal.org/v1/websocket/?login={}.{}&password={}",
                uuid, devide_id, password
            ),
            Environment::STAGING => {
                format!(
                    "wss://chat.staging.signal.org/v1/websocket/?login={}.{}&password={}",
                    uuid, devide_id, password
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config(uuid: &str) -> Config {
        Config {
            user_cfg: UserConfig {
                allowed_uuids: vec![String::from(uuid)],
                ..Default::default()
            },
            ..Default::default()
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
        assert!(cfg.is_url_endpoint_valid(&url::Url::parse("http://0.0.0.0/foo?blah").unwrap()));
        assert!(
            !cfg.is_url_endpoint_valid(&url::Url::parse("http://0.0.0.0:8080/foo?blah").unwrap())
        );
        assert!(!cfg
            .is_url_endpoint_valid(&url::Url::parse("http://user:pass@0.0.0.0/foo?blah").unwrap()));
        assert!(!cfg.is_url_endpoint_valid(&url::Url::parse("https://0.0.0.0/foo?blah").unwrap()));
    }
}
