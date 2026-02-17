use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthConfig {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: String,
    pub vim_mode: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CacheConfig {
    pub max_messages: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub auth: AuthConfig,
    pub ui: UiConfig,
    pub cache: CacheConfig,
}

pub fn load_config() -> Config {
    let config_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config")
        .join("discline")
        .join("config.toml");

    let content = fs::read_to_string(&config_path)
        .unwrap_or_else(|_| panic!("Config not found at {:?}", config_path));

    let config: Config =
        toml::from_str(&content).expect("config.toml has invalid format or missing fields");

    config
}
