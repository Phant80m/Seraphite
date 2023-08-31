use super::{Args, Command};
use crate::linker::{Linker, ToPathbuf};
use crate::utils::{enchant, setup, sync};
use crate::{success, PACKAGE, VERSION};
use anyhow::Result;
use clap::Parser;
use owo_colors::OwoColorize;

impl Args {
    pub fn build() -> Self {
        Args::parse()
    }
    pub fn handle(&self) -> Result<()> {
        if self.version {
            success!("{} build: {}", PACKAGE, VERSION.bold());
            return Ok(());
        }
        match &self.subcommand {
            Some(Command::Tether { dot_dir, config }) => {
                let dot_path = dot_dir
                    .clone()
                    .unwrap_or(format!("{}/dotfiles/", std::env::var("HOME").unwrap()))
                    .path();
                let dot_path = dot_path.join(config.clone().unwrap_or(".config".to_owned()));
                let linker = Linker::new(dot_path, ".config".home_path());
                linker.create_link()?;
            }
            Some(Command::Untether) => {
                let unlinker = Linker::new("dotfiles/.config/".home_path(), ".config".home_path());
                unlinker.remove_link()?;
            }
            Some(Command::Sync { no_confirm }) => {
                sync(*no_confirm)?;
            }
            Some(Command::Setup) => setup()?,
            Some(Command::Enchant { shell }) => {
                enchant(shell.clone().unwrap_or("bash".to_owned()))?
            }
            None => {}
        }
        Ok(())
    }
}
