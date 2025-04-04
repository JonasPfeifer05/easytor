use anyhow::{Context, Result};
use log::{debug, info};
use rust_mkp224o::{build_mkp244o, clone_mkp244o, data_directory};
use std::fs;

fn main() -> Result<()> {
    init_logger();

    info!("Starting to install mkp224o");

    let data_directory =
        data_directory().context("Failed to retrieve the path for the mkp224o data directory")?;
    debug!(
        "Data directory path for mkp224o was retrieved: {:?}",
        data_directory
    );

    fs::create_dir_all(&data_directory).context("Failed to create the mkp224o data directory")?;
    debug!(
        "Data directory for mkp224o was created or already existed: {:?}",
        data_directory
    );

    info!("Starting to download the mkp224o repository...");
    clone_mkp244o(&data_directory).context("Failed to clone the mkp224o repository")?;
    info!("Finished downloading the mkp224o repository");

    info!("Starting to build mkp224o...");

    build_mkp244o(&data_directory).context("Failed to build mkp224o")?;
    info!("mkp224o was successfully built");

    Ok(())
}

fn init_logger() {
    if let Ok(alternative_log_env) = std::env::var("LOG") {
        std::env::set_var("RUST_LOG", alternative_log_env);
    }

    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .format_target(false)
        .init();
}
