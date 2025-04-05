use anyhow::{Context, Result};
use log::{debug, info};
use rust_mkp224o::constants::{
    INTERNAL_MKP224O_EXECUTABLE_PATH, INTERNAL_MKP224O_SOURCE_PATH, MKP224O_REPOSITORY_URL,
};
use rust_mkp224o::repository::git::GitRepository;
use rust_mkp224o::repository::SourceRepository;
use rust_mkp224o::data_directory;
use std::fs;

pub fn install() -> Result<()> {
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

    let repository = GitRepository::new(
        MKP224O_REPOSITORY_URL,
        data_directory.join(INTERNAL_MKP224O_SOURCE_PATH),
        data_directory.join(INTERNAL_MKP224O_EXECUTABLE_PATH),
    );

    info!("Starting to cleanup previous mkp224o installation...");
    repository.cleanup_executable().context("Failed to cleanup previous mkp224o installation")?;
    repository.cleanup_sources().context("Failed to cleanup previous mkp224o installation")?;
    info!("mkp224o was successfully cleaned up");

    info!("Starting to download the mkp224o repository...");
    repository.clone().context("Failed to clone the mkp224o repository")?;
    info!("Finished downloading the mkp224o repository");

    info!("Starting to build mkp224o...");
    repository.build().context("Failed to build mkp224o")?;
    info!("mkp224o was successfully built");

    info!("Starting to cleanup the mkp224o installation...");
    repository.cleanup_sources().context("Failed to cleanup the mkp224o installation")?;
    info!("mkp224o was successfully cleaned up");
    
    Ok(())
}
