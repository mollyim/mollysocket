use serde::{Deserialize, Serialize};
use std::{default::Default, fmt::Debug};

#[derive(Debug, Serialize, Deserialize)]
pub enum Environment {
    STAGING,
    PROD,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    pub environment: Environment,
    pub allowed_endpoints: Vec<String>,
    pub allowed_uuids: Vec<String>,
}

/// `MyConfig` implements `Default`
impl Default for UserConfig {
    fn default() -> Self {
        Self {
            environment: Environment::PROD,
            allowed_endpoints: vec![String::from("http://0.0.0.0/")],
            allowed_uuids: vec![String::from("*")],
        }
    }
}
impl UserConfig {
    pub fn load() -> Result<UserConfig, confy::ConfyError> {
        let cfg: UserConfig = confy::load("mollysocket", None)?;
        Ok(cfg)
    }
}
