use crate::config::load_config;
use anyhow::{Context, Result};

pub mod config;
pub mod errors;

fn main() -> Result<()> {
    let _config = load_config().context("Failed to initialize application configuration")?;

    Ok(())
}
