use crate::constants::{INTERNAL_MKP224O_EXECUTABLE_PATH, INTERNAL_MKP224O_SOURCE_PATH};
use crate::data_directory;
use crate::error::RepositoryError;
use crate::repository::SourceRepository;
use crate::shell::build_command::{BuildCommand, BuildCommandArguments};
use crate::shell::cleanup_command::{RemoveArguments, RemoveDirectoryCommand, RemoveFileCommand};
use crate::shell::command::Command;
use crate::shell::executor::CommandExecutor;
use anyhow::bail;
use git2::Repository;
use std::path::{Path, PathBuf};

pub struct GitRepository {
    url: String,
    sources_path: PathBuf,
    executable_path: PathBuf,
}

impl GitRepository {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn sources_path(&self) -> &Path {
        &self.sources_path
    }

    pub fn executable_path(&self) -> &Path {
        &self.executable_path
    }
}

impl SourceRepository for GitRepository {
    fn new<S: Into<String>, P1: AsRef<Path>, P2: AsRef<Path>>(
        url: S,
        sources_path: P1,
        executable_path: P2,
    ) -> Self
    where
        Self: Sized,
    {
        Self {
            url: url.into(),
            sources_path: sources_path.as_ref().to_path_buf(),
            executable_path: executable_path.as_ref().to_path_buf(),
        }
    }

    fn clone(&self) -> anyhow::Result<()> {
        if self.sources_path.exists() {
            bail!(RepositoryError::SourcesAlreadyExistError(
                self.sources_path.clone()
            ))
        }

        if let Err(err) = Repository::clone(&self.url, &self.sources_path) {
            bail!(RepositoryError::CloneError(anyhow::Error::from(err)))
        }

        Ok(())
    }

    fn is_cloned(&self) -> bool {
        self.sources_path.exists()
    }

    fn build(&self) -> anyhow::Result<()> {
        let data_directory = data_directory()?;
        CommandExecutor::execute_command(BuildCommand::new(BuildCommandArguments {
            sources_directory: data_directory.join(INTERNAL_MKP224O_SOURCE_PATH),
            target_path: data_directory.join(INTERNAL_MKP224O_EXECUTABLE_PATH),
        }))?;

        Ok(())
    }

    fn cleanup_sources(&self) -> anyhow::Result<()> {
        CommandExecutor::execute_command(RemoveDirectoryCommand::new(RemoveArguments {
            path: self.sources_path().to_path_buf(),
        }))?;
        Ok(())
    }

    fn cleanup_executable(&self) -> anyhow::Result<()> {
        CommandExecutor::execute_command(RemoveFileCommand::new(RemoveArguments {
            path: self.executable_path().to_path_buf(),
        }))?;
        Ok(())
    }
}
