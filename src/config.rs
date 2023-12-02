use directories::ProjectDirs;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process;
use std::{default::Default, env, fmt::Debug, sync::OnceLock};

use crate::utils::post_allowed::ResolveAllowed;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize)]
pub enum SignalEnvironment {
    Production,
    Staging,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub signal_env: SignalEnvironment,
    pub allowed_endpoints: Vec<String>,
    pub allowed_uuids: Vec<String>,
    pub db: String,
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

pub fn get_cfg() -> &'static Config {
    CONFIG.get().expect("Config is not initialized yet.")
}

pub fn print() {
    let cfg = get_cfg();
    println!("{:#?}", cfg);
}

pub fn load_config(cli_config_path: Option<PathBuf>) {
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
        Ok(config) => {
            let res = CONFIG.set(config);
            match res {
                Ok(()) => {}
                Err(_err) => {
                    log::error!("Config set error");
                    process::exit(0x0001);
                }
            }
        }
        Err(figment_err) => {
            for err in figment_err {
                log::error!("Config parse error: {}", err);
            }
            process::exit(0x0001);
        }
    }
}

fn get_config_path(cli_config_path: Option<PathBuf>) -> Option<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();

    // from cli argument
    if let Some(cli_path) = cli_config_path {
        paths.push(cli_path)
    }

    // from environment variable
    if let Some(env_path) = env::var_os("MOLLY_CONF") {
        paths.push(env_path.into())
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

// impl Config {
pub fn is_uuid_valid(uuid: &str) -> bool {
    get_cfg()
        .allowed_uuids
        .iter()
        .any(|allowed| allowed == "*" || allowed == uuid)
}

pub async fn is_endpoint_valid(url: &str) -> bool {
    if let Ok(url) = url::Url::parse(url) {
        return is_url_endpoint_valid(&url).await;
    }
    false
}

pub async fn is_url_endpoint_valid(url: &url::Url) -> bool {
    is_endpoint_allowed_by_user(url)
        || (get_cfg().allowed_endpoints.contains(&String::from("*"))
            && url.resolve_allowed().await.unwrap_or(vec![]).len().gt(&0))
}

pub fn is_endpoint_allowed_by_user(url: &url::Url) -> bool {
    get_cfg().allowed_endpoints.iter().any(|allowed| {
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

pub fn get_ws_endpoint(uuid: &str, devide_id: u32, password: &str) -> String {
    match get_cfg().signal_env {
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
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config(uuid: &str) {
        let c = Config {
            allowed_uuids: vec![String::from(uuid)],
            allowed_endpoints: vec![
                String::from("https://ntfy.sh/"),
                String::from("https://up.conversations.im/"),
                String::from("https://fcm.distributor.unifiedpush.org/"),
            ],
            ..Default::default()
        };
        let _ = CONFIG.set(c);
    }

    #[test]
    fn check_wildcard_uuid() {
        test_config("*");
        assert!(is_uuid_valid("0d2ff653-3d88-43de-bcdb-f6657d3484e4"));
    }

    #[test]
    fn check_defined_uuid() {
        test_config("0d2ff653-3d88-43de-bcdb-f6657d3484e4");
        assert!(is_uuid_valid("0d2ff653-3d88-43de-bcdb-f6657d3484e4"));
        assert!(!is_uuid_valid("11111111-3d88-43de-bcdb-f6657d3484e4"));
    }

    #[tokio::test]
    async fn check_endpoint() {
        test_config("*");
        assert!(is_url_endpoint_valid(&url::Url::parse("https://ntfy.sh/foo?blah").unwrap()).await);
        assert!(
            !is_url_endpoint_valid(&url::Url::parse("https://ntfy.sh:8080/foo?blah").unwrap())
                .await
        );
        assert!(
            !is_url_endpoint_valid(&url::Url::parse("https://user:pass@ntfy.sh/foo?blah").unwrap())
                .await
        );
        assert!(!is_url_endpoint_valid(&url::Url::parse("http://ntfy.sh/foo?blah").unwrap()).await);
    }
}
