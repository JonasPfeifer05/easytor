use anyhow::{bail, Context, Result};
use clap::{ColorChoice, CommandFactory, Parser, Subcommand};
use log::{debug, info};
use rust_mkp224o::{build_mkp244o, cleanup_mkp244o, clone_mkp244o, data_directory, execute_mkp244o_command};
use std::fs;
use std::io::{stdout, Write};
use std::process::{exit, Command};
use rust_mkp224o::constants::INTERNAL_MKP224O_EXECUTABLE_PATH;

#[derive(Parser)]
#[command(
    name = "rust-mkp224o",
    about = "A wrapper for mkp224o installation and execution",
    version = "0.1.0"
)]
#[clap(color = concolor_clap::color_choice())]
#[clap(group = clap::ArgGroup::new("command").required(true))]
struct Cli {
    #[arg(trailing_var_arg = true, value_name = "COMMAND", num_args = 0.., help="Passes the command after the \"--\" to the mkp224o executable", group = "command")]
    command_args: Option<Vec<String>>,

    #[arg(
        short = 'i',
        long = "install",
        help = "Installs the newest version of mkp224o by overriding the old installation.",
        group = "command"
    )]
    install: bool,

    #[arg(
        short = 'u',
        long = "update",
        help = "Updates mkp224o to the newest version. Alias for -i or --install.",
        group = "command"
    )]
    update: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    init_logger();
    if let Some(command) = cli.command_args {
        execute_command(command)?
    } else if cli.install || cli.update {
        install()?
    } else {
        bail!("Why are you here? This should not be possible?")
    }
    
    Ok(())
}

fn install() -> Result<()> {
    info!("Starting to install mkp224o");
    
    let data_directory =
        data_directory().context("Failed to retrieve the path for the mkp224o data directory")?;
    debug!(
        "Data directory path for mkp224o was retrieved: {:?}",
        data_directory
    );

    info!("Starting to cleanup mkp224o...");
    cleanup_mkp244o(&data_directory).context("Failed to cleanup mkp224o")?;
    info!("mkp224o was successfully cleaned up");

    fs::create_dir_all(&data_directory).context("Failed to create the mkp224o data directory")?;
    debug!(
        "Data directory for mkp224o was created or already existed: {:?}",
        data_directory
    );

    info!("Starting to download the mkp224o repository...");
    clone_mkp244o(&data_directory).context("Failed to clone the mkp224o repository")?;
    info!("Finished downloading the mkp224o repository");

    info!("Starting to build mkp224o...");
    build_mkp244o(&data_directory).context("Failed to build mkp224o")?;
    info!("mkp224o was successfully built");

    Ok(())
}

fn execute_command(command: Vec<String>) -> Result<()> {
    let data_directory =
        data_directory().context("Failed to retrieve the path for the mkp224o data directory")?;
    debug!(
        "Data directory path for mkp224o was retrieved: {:?}",
        data_directory
    );

    if fs::exists(data_directory.join(INTERNAL_MKP224O_EXECUTABLE_PATH)).context("Failed to check if mkp224o executable exists")? {
        print!("{}", execute_mkp244o_command(command, data_directory)
            .context("Failed to execute mkp244o command")?);
        stdout().lock().flush().context("Failed to flush stdout")?;
    } else {
        println!("mkp224o was not installed and built jet! Please install it using --install!");
    }
    
    Ok(())
}

fn init_logger() {
    if let Ok(alternative_log_env) = std::env::var("LOG") {
        std::env::set_var("RUST_LOG", alternative_log_env);
    }

    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .format_target(false)
        .init();
}
