use crate::error::CommandError::UnsupportedExecutionPlatformError;
use crate::shell::command::Command;
use anyhow::{bail, Result};

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

    fn linux_command(&self) -> Result<String> {
        if which::which("apt-get").is_ok() {
            Ok("sudo apt install gcc libc6-dev libsodium-dev make autoconf -y".to_string())
        } else if which::which("dnf").is_ok() {
            Ok("sudo dnf install gcc glibc-devel libsodium-devel make autoconf -y".to_string())
        } else if which::which("pacman").is_ok() {
            Ok("sudo pacman -S gcc glibc libsodium make autoconf --noconfirm".to_string())
        } else if which::which("emerge").is_ok() {
            Ok("sudo emerge sys-devel/gcc sys-libs/glibc dev-libs/libsodium sys-devel/make sys-devel/autoconf --ask n".to_string())
        } else if which::which("zypper").is_ok() {
            Ok("sudo zypper install gcc glibc libsodium make autoconf -y".to_string())
        } else if which::which("apk").is_ok() {
            Ok("sudo apk add gcc musl-dev libsodium-dev make autoconf --no-interactive".to_string())
        } else {
            // TODO get platform
            bail!(UnsupportedExecutionPlatformError("".to_string()));
        }
    }

    fn macos_command(&self) -> String {
        "brew install gcc libsodium make autoconf".to_string()
    }
}
