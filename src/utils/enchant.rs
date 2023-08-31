use crate::{
    cmd, error,
    linker::{Linker, ToPathbuf},
    success, warning,
};
use anyhow::Result;
use owo_colors::OwoColorize;

use super::sync;

pub fn enchant(shell: String) -> Result<()> {
    // link and install deps
    sync(false)?;
    success!("Dependencies should now be installed!");
    let linker = Linker::new("dotfiles/.config".home_path(), ".config".home_path());
    linker.create_link()?;
    success!("Running post install scripts now!");
    warning!("running post install script with {}", shell.bold());
    let post_install_script = "dotfiles/post_install.sh".home_path();
    if !post_install_script.exists() {
        error!(
            "post install script does not exist! create it at: {}",
            post_install_script.display().bold()
        );
        return Ok(());
    }
    // run post install script!
    cmd!([
        &shell,
        &"dotfiles/post_install.sh".home_path().display().to_string()
    ]);
    Ok(())
}
