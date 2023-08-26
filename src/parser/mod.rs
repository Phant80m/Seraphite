use clap::{Parser, Subcommand};
mod arguments;
#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[clap(name = "tether")]
    /// -> Link dotfiles into the config directory
    Tether,
    #[clap(name = "untether")]
    /// -> unink dotfiles from the config directory
    Untether,
    #[clap(name = "sync")]
    /// -> install dependencies from dependency file
    Sync {
        #[clap(long)]
        no_confirm: bool,
    },
}
