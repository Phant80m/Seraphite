use super::{Args, Command};
use crate::linker::{Linker, ToPathbuf};
use crate::utils::{enchant, setup, sync};
use crate::{success, PACKAGE, VERSION};
use anyhow::Result;
use clap::{Command as Cmd, CommandFactory, Parser};
use clap_complete::{generate, Generator};
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
        if let Some(generator) = self.shell_completion {
            let mut cmd = Self::command();
            eprintln!("Generating completion file for {generator:?}...");
            self.gen_completions(generator, &mut cmd);
        }
        Ok(())
    }
    fn gen_completions<G: Generator>(&self, gen: G, cmd: &mut Cmd) {
        use std::io;
        generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
    }
}
