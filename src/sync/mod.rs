use crate::linker::ToPathbuf;
use crate::{error, warning};
use owo_colors::OwoColorize;
use std::fs;
use std::process::{Command, Stdio};
use which::which;

pub fn sync_deps(no_confirm: bool) {
    let dependencies_file = "dotfiles/dependencies".home_path();
    if !dependencies_file.exists() {
        error!("Dependency file not found!");
        return;
    }

    let paru = match which("paru") {
        Ok(paru_path) => paru_path,
        Err(_) => {
            error!("Paru not found in path, is it installed?");
            return;
        }
    };

    let file = match fs::read_to_string(&dependencies_file) {
        Ok(content) => content,
        Err(err) => {
            error!("Error reading dependency file: {}", err);
            return;
        }
    };

    let no_confirm_args = if no_confirm {
        vec!["--noconfirm"]
    } else {
        Vec::new()
    };

    for line in file.lines() {
        let cmd = Command::new(&paru)
            .args(["-S", "--needed"])
            .args(no_confirm_args.iter())
            .arg(line)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn();

        match cmd {
            Ok(mut child) => {
                let status = child.wait();
                if let Ok(exit_status) = status {
                    warning!(
                        "Exited with status: {}",
                        exit_status.to_string().yellow().bold()
                    );
                } else if let Err(err) = status {
                    error!("Error waiting for process: {}", err);
                }
            }
            Err(err) => {
                error!("Error spawning process: {}", err);
            }
        }
    }
}
