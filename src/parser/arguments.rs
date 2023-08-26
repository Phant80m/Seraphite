use super::{Args, Command};
use crate::linker::{Linker, ToPathbuf};
use crate::sync::sync_deps;
use clap::Parser;
use std::io;

impl Args {
    pub fn build() -> Self {
        Args::parse()
    }
    pub fn handle(&self, dbg: bool) -> io::Result<()> {
        if dbg {
            println!("Input path: {:?}", "./tests/source".home_path());
            println!("Destination path: {:?}", "test/config".home_path());
        }
        let linker = Linker::parse("dotfiles/.config/".home_path(), ".config".home_path());
        match self.subcommand {
            Command::Tether => {
                linker.create_link()?;
            }
            Command::Untether => {
                linker.remove_link()?;
            }
            Command::Sync { no_confirm } => {
                sync_deps(no_confirm);
            }
        }
        Ok(())
    }
}
