use crate::error::CommandError;
use crate::shell::command::Command;
use anyhow::bail;
use anyhow::Result;
use std::process::Command as StdCommand;

pub struct CommandExecutor;

impl CommandExecutor {
    /// Runs a command in an os process
    pub fn execute_command<C: Command>(command: C) -> Result<String> {
        let mut process = if cfg!(target_os = "windows") {
            Self::windows_command()
        } else if cfg!(target_os = "macos") {
            Self::macos_command()
        } else if cfg!(target_os = "linux") {
            Self::linux_command()
        } else {
            // TODO Retrieve platform string
            bail!(CommandError::UnsupportedExecutionPlatformError(
                "unknown".into()
            ))
        };

        let output = process
            .arg(command.command()?)
            .output()
            .map_err(|err| CommandError::CommandExecutionFailedError(err.to_string()))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            bail!(CommandError::CommandExecutionFailedError(stderr))
        }
    }

    /// Command to start a windows process
    fn windows_command() -> StdCommand {
        let mut command = StdCommand::new("cmd");
        command.arg("/C");
        command
    }

    /// Command to start a linux command
    fn linux_command() -> StdCommand {
        Self::unix_command()
    }

    /// Command to start a macos command
    fn macos_command() -> StdCommand {
        Self::unix_command()
    }

    /// Command to start a unix command
    fn unix_command() -> StdCommand {
        let mut command = StdCommand::new("sh");
        command.arg("-c");
        command
    }
}
