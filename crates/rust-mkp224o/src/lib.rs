use crate::constants::APPLICATION_NAME;
use anyhow::{Context, Result};
use std::path::PathBuf;

pub mod constants;
pub mod error;
pub mod repository;
pub mod shell;

pub fn data_directory() -> Result<PathBuf> {
    let data_prefix = dirs::data_dir().context("Could not retrieve data directory for this os")?;
    Ok(data_prefix.join(APPLICATION_NAME))
}
