use anyhow::Result;

pub trait Command {
    type Arguments;

    /// Creates an new command with the provided arguments
    ///
    /// # Arguments
    /// * `arguments` - Arguments for the command
    fn new(arguments: Self::Arguments) -> Self;

    /// Gets the command for the correct platform
    fn command(&self) -> Result<String> {
        if cfg!(target_os = "windows") {
            Ok(self.windows_command())
        } else if cfg!(target_os = "linux") {
            self.linux_command()
        } else
        /*macos*/
        {
           Ok(self.macos_command())
        }
    }

    /// Gets the windows command
    fn windows_command(&self) -> String;

    /// Gets the linux command
    fn linux_command(&self) -> Result<String>;

    /// Gets the macos command
    fn macos_command(&self) -> String;
}
