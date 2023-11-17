use serde::{Deserialize, Serialize};
use std::{default::Default, env, fmt::Debug};

#[derive(Debug, Serialize, Deserialize)]
pub enum Environment {
    Staging,
    Prod,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    pub environment: Environment,
    pub allowed_endpoints: Vec<String>,
    pub allowed_uuids: Vec<String>,
    pub db: String,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            environment: Environment::Prod,
            allowed_endpoints: vec![
                String::from("https://ntfy.sh/"),
                String::from("https://up.conversations.im/"),
                String::from("https://fcm.distributor.unifiedpush.org/"),
            ],
            allowed_uuids: vec![String::from("*")],
            db: String::from("./mollysocket.db"),
        }
    }
}
impl UserConfig {
    pub fn load() -> Result<UserConfig, confy::ConfyError> {
        let cfg: UserConfig = if let Some(path) = env::var_os("MOLLY_CONF") {
            confy::load_path(path)?
        } else {
            confy::load("mollysocket", None)?
        };
        Ok(cfg)
    }
}
