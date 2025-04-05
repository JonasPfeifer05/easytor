use anyhow::{Context, Result};
use log::{debug, info};
use rust_mkp224o::constants::{
    INTERNAL_MKP224O_EXECUTABLE_PATH, INTERNAL_MKP224O_SOURCE_PATH, MKP224O_REPOSITORY_URL,
};
use rust_mkp224o::data_directory;
use rust_mkp224o::repository::git::GitRepository;
use rust_mkp224o::repository::SourceRepository;

pub fn uninstall() -> Result<()> {
    info!("Starting to uninstall mkp224o");

    let data_directory =
        data_directory().context("Failed to retrieve the path for the mkp224o data directory")?;
    debug!(
        "Data directory path for mkp224o was retrieved: {:?}",
        data_directory
    );

    let repository = GitRepository::new(
        MKP224O_REPOSITORY_URL,
        data_directory.join(INTERNAL_MKP224O_SOURCE_PATH),
        data_directory.join(INTERNAL_MKP224O_EXECUTABLE_PATH),
    );

    info!("Starting to remove mkp224o installation...");
    repository
        .cleanup_executable()
        .context("Failed to remove mkp224o installation")?;
    repository
        .cleanup_sources()
        .context("Failed to remove mkp224o installation")?;
    info!("Mkp224o installation was successfully removed");

    Ok(())
}
