use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::{fs, io::Error};

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

impl Default for Config {
    fn default() -> Self {
        Config {
            auth: AuthConfig {
                token: String::new(),
            },
            ui: UiConfig {
                theme: String::new(),
                vim_mode: false,
            },
            cache: CacheConfig { max_messages: 100 },
        }
    }
}

pub fn load_config() -> Result<Config, Error> {
    let _ = dotenv();

    let config_path = dirs::config_dir().map(|path| path.join("discline/config.toml"));

    let config = if let Some(path) = config_path.filter(|path| path.exists()) {
        let content = fs::read_to_string(&path).unwrap();
        toml::from_str(&content).unwrap()
    } else {
        Config::default()
    };

    Ok(config)
}
