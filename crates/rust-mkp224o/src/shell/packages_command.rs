use crate::shell::command::Command;

pub struct PackageCommand;

impl Command for PackageCommand {
    type Arguments = ();

    fn new(arguments: Self::Arguments) -> Self {
        Self
    }

    fn windows_command(&self) -> String {
        // TODO check for required dependencies
        "".to_string()
    }

    fn linux_command(&self) -> String {
        "apt install -y gcc libc6-dev libsodium-dev make autoconf".to_string()
    }

    fn macos_command(&self) -> String {
        "brew install gcc libsodium make autoconf".to_string()
    }
}
