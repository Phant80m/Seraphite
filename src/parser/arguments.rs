use super::{Args, Command};
use crate::linker::{Linker, ToPathbuf};
use crate::sync::sync_deps;
use anyhow::Result;
use clap::Parser;

impl Args {
    pub fn build() -> Self {
        Args::parse()
    }
    pub fn handle(&self, dbg: bool) -> Result<()> {
        if dbg {
            println!("Input path: {:?}", "./tests/source".home_path());
            println!("Destination path: {:?}", "test/config".home_path());
        }
        match &self.subcommand {
            Command::Tether { dot_dir, config } => {
                let dot_path = dot_dir
                    .clone()
                    .unwrap_or(format!("{}/dotfiles/", std::env::var("HOME").unwrap()))
                    .path();
                let dot_path = dot_path.join(config.clone().unwrap_or(".config".to_owned()));
                let linker = Linker::new(dot_path, ".config".home_path());
                linker.create_link()?;
            }
            Command::Untether => {
                let linker = Linker::new("dotfiles/.config/".home_path(), ".config".home_path());
                linker.remove_link()?;
            }
            Command::Sync { no_confirm } => {
                sync_deps(*no_confirm);
            }
        }
        Ok(())
    }
}
