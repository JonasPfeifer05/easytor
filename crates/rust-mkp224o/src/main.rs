mod cli;
mod commands;
mod logger;

use crate::cli::Mkp224oCli;
use crate::commands::execute::execute_mkp224o_command;
use crate::commands::install::install;
use crate::commands::uninstall::uninstall;
use crate::logger::Logger;
use anyhow::{bail, Result};
use clap::Parser;
use rust_mkp224o::constants::DEFAULT_LOG_LEVEL;

fn main() -> Result<()> {
    let cli = Mkp224oCli::parse();

    Logger::init_with_default_level(DEFAULT_LOG_LEVEL);
    if let Some(command) = cli.command_args {
        execute_mkp224o_command(command)?
    } else if cli.install || cli.update {
        install()?
    } else if cli.remove {
        uninstall()?
    } else {
        bail!("Why are you here? This should not be possible? Goodbye 🫡")
    }

    Ok(())
}
