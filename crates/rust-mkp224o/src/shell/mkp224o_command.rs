use crate::shell::command::Command;
use shell_escape::escape;
use std::path::PathBuf;

pub struct Mk224oCommand {
    pub arguments: Mk224oCommandArguments,
}

pub struct Mk224oCommandArguments {
    pub executable_path: PathBuf,
    pub command: Vec<String>,
}

impl Command for Mk224oCommand {
    type Arguments = Mk224oCommandArguments;

    fn new(arguments: Self::Arguments) -> Self {
        Self { arguments }
    }

    fn windows_command(&self) -> String {
        self.command()
    }

    fn linux_command(&self) -> String {
        self.command()
    }

    fn macos_command(&self) -> String {
        self.command()
    }
}

impl Mk224oCommand {
    fn command(&self) -> String {
        format!(
            "{} {}",
            escape(self.arguments.executable_path.to_string_lossy()),
            self.arguments.command.join(" ")
        )
    }
}
