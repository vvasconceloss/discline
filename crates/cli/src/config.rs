use serde::{Deserialize, Serialize};

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
