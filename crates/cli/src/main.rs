use crate::config::load_config;
use anyhow::{Context, Result};
use client::client::HttpClient;

pub mod config;
pub mod errors;

fn main() -> Result<()> {
    let config = load_config().context("Failed to initialize application configuration")?;
    let _client = HttpClient::new(config.auth.token);

    Ok(())
}
