use crate::linker::ToPathbuf;
use crate::{error, success, warning};
use anyhow::Result;
use owo_colors::OwoColorize;
use std::fs;

pub fn setup() -> Result<()> {
    let config: String = ".config".home_path().display().to_string();
    let dotdir = "dotfiles".home_path();
    if "dotfiles/".home_path().exists() {
        error!("Refusing to delete existing dotfile directory");
        return Ok(());
    }
    if let Err(e) = fs::create_dir_all(dotdir.join(".config").canonicalize()?) {
        error!("Failed to create dotfiles skeleton: {}", e);
        return Ok(());
    }
    success!("Created dotfiles directory at: {}", dotdir.display().bold());
    success!(
        "Add your configurations to {}!",
        dotdir.join(".config").display().bold()
    );
    success!(
        "then run {} to stash them to your {}",
        "seraphite tether".on_black().magenta(),
        config.bold()
    );
    warning!(
        "Optional step, create a file at {}, to access the {} command",
        "dotfiles/dependencies".home_path().display().bold(),
        "seraphite sync".on_black().magenta()
    );
    Ok(())
}
