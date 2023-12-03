use directories::ProjectDirs;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::{env, fmt::Debug, path::PathBuf, process, sync::OnceLock};

use crate::utils::post_allowed::ResolveAllowed;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize)]
pub enum SignalEnvironment {
    Production,
    Staging,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    host: String,
    port: u16,
    signal_env: SignalEnvironment,
    allowed_endpoints: Vec<String>,
    allowed_uuids: Vec<String>,
    db: String,
}

#[derive(Debug, PartialEq, Eq)]
enum EndpointValidity {
    Ok,
    NotInConfig,
    Private,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port: 8020,
            signal_env: SignalEnvironment::Production,
            allowed_endpoints: vec![String::from("*")],
            allowed_uuids: vec![String::from("*")],
            db: String::from("./mollysocket.db"),
        }
    }
}

fn get_cfg() -> &'static Config {
    CONFIG.get().expect("Config is not initialized yet.")
}

pub fn get_db() -> String {
    get_cfg().db.clone()
}

pub fn get_host() -> String {
    get_cfg().host.clone()
}

pub fn get_port() -> u16 {
    get_cfg().port
}

pub fn is_uuid_valid(uuid: &str) -> bool {
    get_cfg().is_uuid_valid(uuid)
}

pub fn get_ws_endpoint(uuid: &str, devide_id: u32, password: &str) -> String {
    get_cfg().get_ws_endpoint(uuid, devide_id, password)
}

pub async fn is_endpoint_valid(url: &str) -> bool {
    get_cfg().is_endpoint_valid(url).await
}

pub fn is_endpoint_allowed_by_user(url: &url::Url) -> bool {
    get_cfg().is_endpoint_allowed_by_user(url)
}

pub fn print() {
    let cfg = get_cfg();
    println!("{:#?}", cfg)
}

pub fn load_config(cli_config_path: Option<PathBuf>) {
    CONFIG.get_or_init(move || {
        let mut figment = Figment::new();

        figment = figment.merge(Serialized::defaults(Config::default()));

        if let Some(path) = get_config_path(cli_config_path) {
            log::info!("Config file: {}", path.display());
            figment = figment.merge(Toml::file(path));
        } else {
            log::info!("No config file supplied");
        }

        figment = figment.merge(Env::prefixed("MOLLY_").ignore(&["conf"]));

        match figment.extract() {
            Ok(config) => config,
            Err(figment_err) => {
                for err in figment_err {
                    log::error!("Config parse error: {}", err);
                }
                process::exit(0x0001);
            }
        }
    });
}

fn get_config_path(cli_config_path: Option<PathBuf>) -> Option<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();

    // from cli argument
    if let Some(cli_path) = cli_config_path {
        if cli_path.exists() {
            return Some(cli_path);
        } else {
            panic!("{} not found.", cli_path.display());
        }
    }

    // from environment variable
    if let Some(env_path) = env::var_os("MOLLY_CONF") {
        let path = Into::<PathBuf>::into(env_path);
        if path.exists() {
            return Some(path);
        } else {
            panic!("MOLLY_CONF={}, file not found.", path.display());
        }
    }

    // from xdg_config_home
    let proj_dirs = ProjectDirs::from("org", "mollyim", "mollysocket").unwrap();
    paths.push(proj_dirs.config_dir().join("config.toml"));

    // in current directory
    paths.push(PathBuf::from("./mollysocket.toml"));

    // in linux /etc dir
    if cfg!(target_os = "linux") {
        paths.push(PathBuf::from("/etc/mollysocket/config.toml"));
    }

    for p in paths.iter() {
        if p.exists() {
            return Some(p.to_path_buf());
        }
    }
    None
}

impl Config {
    fn is_uuid_valid(&self, uuid: &str) -> bool {
        self.allowed_uuids
            .clone()
            .iter()
            .any(|allowed| allowed == "*" || allowed == uuid)
    }

    fn endpoint_to_conf(&self, url: &url::Url) -> String {
        let mut conf_url = url::Url::parse("http://example.tld/").unwrap();
        let _ = conf_url.set_scheme(url.scheme());
        let _ = conf_url.set_host(url.host_str());
        let _ = conf_url.set_port(url.port());
        let _ = conf_url.set_username(url.username());
        let _ = conf_url.set_password(url.password());
        conf_url.into()
    }

    async fn is_endpoint_valid(&self, url: &str) -> bool {
        if let Ok(url) = url::Url::parse(url) {
            let endpoint_validity = self.is_url_endpoint_valid(&url).await;
            match endpoint_validity {
                EndpointValidity::Ok => true,
                EndpointValidity::NotInConfig => {
                    log::warn!(
                        "Endpoint not allowed: {}\n\
You may want to add \"{}\" to allowed_endpoints",
                        url,
                        self.endpoint_to_conf(&url)
                    );
                    false
                }
                EndpointValidity::Private => {
                    log::warn!(
                        "Endpoint resolves to a private IP: {}\n\
You may want to add \"{}\" to allowed_endpoints",
                        url,
                        self.endpoint_to_conf(&url)
                    );
                    false
                }
            }
        } else {
            false
        }
    }

    fn get_ws_endpoint(&self, uuid: &str, devide_id: u32, password: &str) -> String {
        match self.signal_env {
            SignalEnvironment::Production => format!(
                "wss://chat.signal.org/v1/websocket/?login={}.{}&password={}",
                uuid, devide_id, password
            ),
            SignalEnvironment::Staging => {
                format!(
                    "wss://chat.staging.signal.org/v1/websocket/?login={}.{}&password={}",
                    uuid, devide_id, password
                )
            }
        }
    }
    async fn is_url_endpoint_valid(&self, url: &url::Url) -> EndpointValidity {
        if self.is_endpoint_allowed_by_user(url) {
            EndpointValidity::Ok
        } else {
            if self.allowed_endpoints.contains(&"*".into()) {
                if url.resolve_allowed().await.unwrap_or(vec![]).len().gt(&0) {
                    EndpointValidity::Ok
                } else {
                    EndpointValidity::Private
                }
            } else {
                EndpointValidity::NotInConfig
            }
        }
    }

    fn is_endpoint_allowed_by_user(&self, url: &url::Url) -> bool {
        self.allowed_endpoints.iter().any(|allowed| {
            if let Ok(allowed_url) = url::Url::parse(allowed) {
                url.host() == allowed_url.host()
                    && url.port() == allowed_url.port()
                    && url.scheme() == allowed_url.scheme()
                    && url.username() == allowed_url.username()
                    && url.password() == allowed_url.password()
            } else {
                false
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config(uuid: &str, endpoint: &str) -> Config {
        let allowed_uuids = vec![String::from(uuid)];
        let allowed_endpoints = vec![String::from(endpoint)];
        dbg!(Config {
            allowed_endpoints,
            allowed_uuids,
            ..Config::default()
        })
    }

    #[test]
    fn check_wildcard_uuid() {
        let cfg = test_config("*", "");
        assert!(cfg.is_uuid_valid("0d2ff653-3d88-43de-bcdb-f6657d3484e4"));
    }

    #[test]
    fn check_defined_uuid() {
        let cfg = test_config("0d2ff653-3d88-43de-bcdb-f6657d3484e4", "");
        assert!(cfg.is_uuid_valid("0d2ff653-3d88-43de-bcdb-f6657d3484e4"));
        assert!(!cfg.is_uuid_valid("11111111-3d88-43de-bcdb-f6657d3484e4"));
    }

    #[tokio::test]
    async fn check_endpoint() {
        let cfg = test_config("", "https://ntfy.sh/");
        assert_eq!(
            cfg.is_url_endpoint_valid(&url::Url::parse("https://ntfy.sh/foo?blah").unwrap())
                .await,
            EndpointValidity::Ok
        );
        assert_eq!(
            cfg.is_url_endpoint_valid(&url::Url::parse("https://ntfy.sh:8080/foo?blah").unwrap())
                .await,
            EndpointValidity::NotInConfig
        );
        assert_eq!(
            cfg.is_url_endpoint_valid(
                &url::Url::parse("https://user:pass@ntfy.sh/foo?blah").unwrap()
            )
            .await,
            EndpointValidity::NotInConfig
        );
        assert_eq!(
            cfg.is_url_endpoint_valid(&url::Url::parse("http://ntfy.sh/foo?blah").unwrap())
                .await,
            EndpointValidity::NotInConfig
        );
    }

    #[tokio::test]
    async fn check_wildcard_endpoint() {
        let cfg = test_config("", "*");
        assert_eq!(
            cfg.is_url_endpoint_valid(&url::Url::parse("http://ntfy.sh/foo?blah").unwrap())
                .await,
            EndpointValidity::Ok
        );
        assert_eq!(
            cfg.is_url_endpoint_valid(&url::Url::parse("http://localhost/foo?blah").unwrap())
                .await,
            EndpointValidity::Private
        );
    }
}
