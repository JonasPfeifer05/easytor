use anyhow::Context;
use log::debug;
use rust_mkp224o::constants::INTERNAL_MKP224O_EXECUTABLE_PATH;
use rust_mkp224o::data_directory;
use rust_mkp224o::shell::command::Command;
use rust_mkp224o::shell::executor::CommandExecutor;
use rust_mkp224o::shell::mkp224o_command::{Mk224oCommand, Mk224oCommandArguments};
use std::fs;
use std::io::{stdout, Write};

pub fn execute_mkp224o_command(command: Vec<String>) -> anyhow::Result<()> {
    let data_directory =
        data_directory().context("Failed to retrieve the path for the mkp224o data directory")?;
    debug!(
        "Data directory path for mkp224o was retrieved: {:?}",
        data_directory
    );

    if fs::exists(data_directory.join(INTERNAL_MKP224O_EXECUTABLE_PATH))
        .context("Failed to check if mkp224o executable exists")?
    {
        let stdout_string =
            CommandExecutor::execute_command(Mk224oCommand::new(Mk224oCommandArguments {
                executable_path: data_directory.join(INTERNAL_MKP224O_EXECUTABLE_PATH),
                command,
            }))
            .context("Failed to execute mkp244o command")?;

        print!("{}", stdout_string);
        stdout().lock().flush().context("Failed to flush stdout")?;
    } else {
        println!("mkp224o was not installed and built jet! Please install it using --install!");
    }

    Ok(())
}
