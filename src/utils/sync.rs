use crate::cmd;
use crate::error;
use crate::linker::ToPathbuf;
use crate::utils::downloader::download;
use crate::warning;
use anyhow::Result;
use owo_colors::OwoColorize;
use std::fs;
use which::which;
pub fn sync(no_confirm: bool) -> Result<()> {
    let dependencies_file = "dotfiles/dependencies".home_path();
    if !dependencies_file.exists() {
        error!("Dependency file not found!");
        return Ok(());
    }

    match which("paru") {
        Ok(paru_path) => paru_path,
        Err(_) => {
            error!("Paru not found in path, is it installed?");
            warning!("attempting to install paru!");
            download("https://aur.archlinux.org/paru-bin.git")?;
            return Ok(());
        }
    };

    let file = match fs::read_to_string(&dependencies_file) {
        Ok(content) => content,
        Err(err) => {
            error!("Error reading dependency file: {}", err);
            return Ok(());
        }
    };

    let no_confirm_args = if no_confirm {
        vec!["--noconfirm"]
    } else {
        Vec::new()
    };

    for line in file.lines() {
        let mut command = vec!["paru", "-S", "--needed"];
        command.extend(&no_confirm_args);
        command.push(line);

        cmd!(command);
    }
    Ok(())
}
