use crate::linker::ToPathbuf;
use crate::{error, warning};
use owo_colors::OwoColorize;
use std::fs;
use std::process::{Command, Stdio};
use which::which;

pub fn sync_deps(no_confirm: bool) {
    if !"dotfiles/dependencies".home_path().exists() {
        error!("Dependency file not found!");
        return;
    }
    let paru = which("paru").unwrap_or_default();
    if !paru.exists() {
        error!("Paru not found in path, is it installed?");

        return;
    }
    let file = fs::read_to_string("dotfiles/dependencies".home_path()).unwrap();
    if no_confirm {
        for line in file.lines() {
            let mut cmd = Command::new("paru")
                .args(["-S", "--needed", line, "--noconfirm"])
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .unwrap();

            let status = cmd.wait();
            warning!("Exited with status: {}", status.unwrap().yellow().bold());
        }
    }
    for line in file.lines() {
        let mut cmd = Command::new("paru")
            .args(["-S", "--needed", line])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();

        let status = cmd.wait();
        warning!("Exited with status: {}", status.unwrap().yellow().bold());
    }
}
