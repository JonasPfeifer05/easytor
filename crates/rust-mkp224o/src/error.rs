use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("The repository could not be cloned because the sources already existed at path {0}")]
    SourcesAlreadyExistError(PathBuf),

    #[error("Cloning the repository failed: {0}")]
    CloneError(anyhow::Error),

    #[error("Building the repository failed: {0}")]
    BuildError(anyhow::Error),

    #[error("Cleaning up the repository failed: {0}")]
    CleanupError(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Cannot execute command for the platform: {0}")]
    UnsupportedExecutionPlatformError(String),

    #[error("The command could not be executed: {0}")]
    CommandExecutionFailedError(String),
}
