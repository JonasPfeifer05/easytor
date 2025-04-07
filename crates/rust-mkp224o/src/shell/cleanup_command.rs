use crate::shell::command::Command;
use anyhow::Result;
use shell_escape::escape;
use std::path::PathBuf;

pub struct RemoveDirectoryCommand {
    arguments: RemoveArguments,
}

pub struct RemoveFileCommand {
    arguments: RemoveArguments,
}

pub struct RemoveArguments {
    pub path: PathBuf,
}

impl Command for RemoveFileCommand {
    type Arguments = RemoveArguments;

    fn new(arguments: Self::Arguments) -> Self {
        Self { arguments }
    }

    fn windows_command(&self) -> String {
        format!(
            "del /f /q \"{}\"",
            escape(self.arguments.path.to_string_lossy())
        )
    }

    fn linux_command(&self) -> Result<String> {
        Ok(self.unix_command())
    }

    fn macos_command(&self) -> String {
        self.unix_command()
    }
}

impl RemoveFileCommand {
    fn unix_command(&self) -> String {
        format!("rm -f {}", escape(self.arguments.path.to_string_lossy()))
    }
}

impl Command for RemoveDirectoryCommand {
    type Arguments = RemoveArguments;

    fn new(arguments: Self::Arguments) -> Self {
        Self { arguments }
    }

    fn windows_command(&self) -> String {
        format!(
            "rmdir /s /q \"{}\"",
            escape(self.arguments.path.to_string_lossy())
        )
    }

    fn linux_command(&self) -> Result<String> {
        Ok(self.unix_command())
    }

    fn macos_command(&self) -> String {
        self.unix_command()
    }
}

impl RemoveDirectoryCommand {
    fn unix_command(&self) -> String {
        format!("rm -rfd {}", escape(self.arguments.path.to_string_lossy()))
    }
}
