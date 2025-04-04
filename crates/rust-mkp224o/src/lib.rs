use crate::constants::{
    APPLICATION_NAME, INTERNAL_MKP224O_EXECUTABLE_PATH, INTERNAL_MKP224O_SOURCE_PATH,
    MKP224O_REPOSITORY_URL,
};
use anyhow::{bail, Context, Result};
use git2::Repository;
use shell_escape::escape;
use std::path::{Path, PathBuf};
use std::process::Command;

pub mod constants;

pub fn data_directory() -> Result<PathBuf> {
    let data_prefix = dirs::data_dir().context("Could not retrieve data directory for this os")?;
    Ok(data_prefix.join(APPLICATION_NAME))
}

pub fn clone_mkp244o<P: AsRef<Path>>(data_directory: P) -> Result<()> {
    Repository::clone(
        MKP224O_REPOSITORY_URL,
        data_directory.as_ref().join(INTERNAL_MKP224O_SOURCE_PATH),
    )?;
    Ok(())
}

pub fn build_mkp244o<P: AsRef<Path>>(data_directory: P) -> Result<()> {
    let output = if cfg!(target_os = "windows") {
        bail!("Windows is not supported");
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(
                mkp224o_build_command_string(data_directory)
                    .context("Failed to construct command for building mkp224o")?,
            )
            .output()
            .context("Failed to execute build command for mkp224o")?
    };
    if output.status.success() {
        Ok(())
    } else {
        bail!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

fn mkp224o_build_command_string<P: AsRef<Path>>(data_directory: P) -> Result<String> {
    let data_path = data_directory.as_ref();
    let sources_path = escape(
        data_path
            .join(INTERNAL_MKP224O_SOURCE_PATH)
            .to_str()
            .context("Failed to construct path to mkp224o source")?
            .to_string()
            .into(),
    );
    let executable_path = escape(
        data_path
            .join(INTERNAL_MKP224O_EXECUTABLE_PATH)
            .to_str()
            .context("Failed to construct path to mkp224o executable")?
            .to_string()
            .into(),
    );
    let data_path = escape(
        data_path
            .to_str()
            .context("Failed to construct path to mkp224o data directory")?
            .to_string()
            .into(),
    );

    Ok(format!(
        "cd {} &&
export CFLAGS=\"-I/opt/homebrew/include\" &&
export LDFLAGS=\"-L/opt/homebrew/lib\" &&
./autogen.sh &&
./configure &&
make &&
mv ./mkp224o {} &&
cd {} &&
rm -rfd {}
    ",
        sources_path, executable_path, data_path, sources_path
    ))
}
