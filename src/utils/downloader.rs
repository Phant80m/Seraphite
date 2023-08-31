use crate::{cmd, success, warning};
use anyhow::Result;
use git2::Repository;
use owo_colors::OwoColorize;
use std::fs;
use std::process::{Command, Stdio};

use crate::{error, linker::ToPathbuf};
pub fn download(url: &str) -> Result<()> {
    if !"/lib/libgit2.so".path().exists() {
        cmd!(["sudo", "pacman", "-S", "libgit2"]);
    }
    if ".seraphite/cache/paru-bin/".home_path().exists() {
    } else if let Err(e) = fs::create_dir(".seraphite/cache/paru-bin".home_path()) {
        error!("failed to create cache dir! -> {}", e.bold().red());
        return Ok(());
    }
    if ".seraphite/cache/paru-bin/PKGBUILD".home_path().exists() {
    } else if let Err(e) = Repository::clone(url, ".seraphite/cache/paru-bin".home_path()) {
        error!("failed to clone {}, reason: {}", url.bold(), e.bold().red());
        return Ok(());
    }
    let home_dir = std::env::var("HOME")?;

    Command::new("bash")
        .arg("-c")
        .arg(format!(
            "cd {}/.seraphite/cache/paru-bin/; makepkg -si",
            home_dir
        ))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait_with_output()?;

    warning!("Starting cleanup phase!");
    if let Err(e) = fs::remove_dir_all(".seraphite/.cache/paru-bin/".home_path()) {
        error!("failed to remove cache! -> {}", e.bold().red());
        return Ok(());
    }
    success!("successfuly cleaned up!");

    Ok(())
}
