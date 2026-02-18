use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::errors::ConfigError;

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
                theme: "default".to_string(),
                vim_mode: false,
            },
            cache: CacheConfig { max_messages: 100 },
        }
    }
}

pub fn load_config() -> Result<Config, ConfigError> {
    let _ = dotenv();

    let config_path = dirs::config_dir()
        .map(|path| path.join("discline/config.toml"))
        .ok_or(ConfigError::NoConfigDir)?;

    let mut config = if config_path.exists() {
        let content = fs::read_to_string(&config_path)?;
        toml::from_str(&content)?
    } else {
        Config::default()
    };

    if config.auth.token.is_empty()
        && let Ok(token) = std::env::var("DISCORD_TOKEN")
    {
        config.auth.token = token;
    }

    if config.auth.token.is_empty() {
        return Err(ConfigError::MissingToken);
    }

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parsing() {
        let toml_str = r#"
            [auth]
            token = "test-token"

            [ui]
            theme = "dark"
            vim_mode = true

            [cache]
            max_messages = 50
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.auth.token, "test-token");
        assert_eq!(config.ui.theme, "dark");
        assert!(config.ui.vim_mode);
        assert_eq!(config.cache.max_messages, 50);
    }

    #[test]
    fn test_token_precedence_file_wins() {
        let mut config = Config::default();
        config.auth.token = "file-token".to_string();

        unsafe { std::env::set_var("DISCORD_TOKEN", "env-token") };

        if config.auth.token.is_empty() {
            if let Ok(token) = std::env::var("DISCORD_TOKEN") {
                config.auth.token = token;
            }
        }

        assert_eq!(config.auth.token, "file-token");
        unsafe { std::env::remove_var("DISCORD_TOKEN") };
    }

    #[test]
    fn test_token_precedence_env_fallback() {
        let mut config = Config::default();
        config.auth.token = "".to_string();

        unsafe { std::env::set_var("DISCORD_TOKEN", "env-token") };

        if config.auth.token.is_empty() {
            if let Ok(token) = std::env::var("DISCORD_TOKEN") {
                config.auth.token = token;
            }
        }

        assert_eq!(config.auth.token, "env-token");
        unsafe { std::env::remove_var("DISCORD_TOKEN") };
    }
}
