use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse config file: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("Discord token not found in config.toml or DISCORD_TOKEN env var")]
    MissingToken,

    #[error("Failed to determine config directory")]
    NoConfigDir,
}
