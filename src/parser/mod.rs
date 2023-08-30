use clap::{Parser, Subcommand};
mod arguments;
#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
pub struct Args {
    #[arg(short, long)]
    version: bool,
    #[clap(subcommand)]
    pub subcommand: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// -> Setup the dotfiles directory
    #[clap(name = "setup")]
    Setup,
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
    #[clap(name = "enchant")]
    /// -> sync, tether & run post install script
    Enchant {
        #[arg(short, long)]
        /// choose what shell runs post install script
        shell: Option<String>,
    },
}
