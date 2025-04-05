use std::path::Path;
use anyhow::Result;

pub mod git;
pub mod build_command;
pub mod cleanup_command;
pub mod mkp224o_command;

pub trait SourceRepository {
    /// Clone the repository from a URL into a target directory
    ///
    /// # Arguments
    /// * `url` - The repository URL
    /// * `sources_path` - Path to the desired sources directory
    /// * `executable_path` Path to desired executable file
    fn new<S: Into<String>, P1: AsRef<Path>, P2: AsRef<Path>>(
        url: S,
        sources_path: P1,
        executable_path: P2,
    ) -> Self;

    /// Clone the repository from a URL into a target directory
    fn clone(&self) -> Result<()>;

    /// Check if the repository has already been cloned
    fn is_cloned(&self) -> bool;
    
    /// Build the mkp224o executable from the sources
    fn build(&self) -> Result<()>;

    /// Clean up the repository sources directory
    fn cleanup_sources(&self) -> Result<()>;

    /// Clean up the repository executable
    fn cleanup_executable(&self) -> Result<()>;
}
