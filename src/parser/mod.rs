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
    Tether {
        #[arg(short, long)]
        /// -> specify your dotfiles repo location, (is a bit broken)
        dot_dir: Option<String>,
        #[arg(short, long)]
        /// -> specify which config you want to use, inside your dotfile repo
        config: Option<String>,
    },
    #[clap(name = "untether")]
    /// -> unink dotfiles from the config directory
    Untether,
    #[clap(name = "sync")]
    /// -> install dependencies from dependency file
    Sync {
        #[clap(short = 'y', long)]
        no_confirm: bool,
    },
}
