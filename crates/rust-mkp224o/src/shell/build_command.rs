use crate::shell::command::Command;
use shell_escape::escape;
use std::path::PathBuf;

pub struct BuildCommand {
    arguments: BuildCommandArguments,
}

pub struct BuildCommandArguments {
    pub sources_directory: PathBuf,
    pub target_path: PathBuf,
}

impl Command for BuildCommand {
    type Arguments = BuildCommandArguments;

    fn new(arguments: Self::Arguments) -> Self {
        Self { arguments }
    }

    fn windows_command(&self) -> String {
        todo!("Windows is not supported")
    }

    fn linux_command(&self) -> String {
        format!(
            "cd {} &&
./autogen.sh &&
./configure &&
make &&
mv ./mkp224o {}
",
            escape(self.arguments.sources_directory.to_string_lossy()),
            escape(self.arguments.target_path.to_string_lossy())
        )
    }

    fn macos_command(&self) -> String {
        format!(
            "cd {} &&
export CFLAGS=\"-I/opt/homebrew/include\" &&
export LDFLAGS=\"-L/opt/homebrew/lib\" &&
./autogen.sh &&
./configure &&
make &&
mv ./mkp224o {}
",
            escape(self.arguments.sources_directory.to_string_lossy()),
            escape(self.arguments.target_path.to_string_lossy())
        )
    }
}
