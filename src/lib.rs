use owo_colors::OwoColorize;

pub fn help() -> String {
    let help: String = format!(
        "{}\n\n{} {} {}\n\n{}\n    {} {} {}\n    {} {} {}{}{}\n    {} {} {}{}{}",
        "[ Seraphite 💎 ]".bold().bright_cyan(),
        "Usage:".bold().green().underline(),
        "seraphite".blue(),
        "<COMMAND>".magenta(),
        "Commands:".green().underline().bold(),
        "help".magenta(),
        "->".red(),
        "Displays the help command.".yellow(),
        "stash".magenta(),
        "->".red(),
        "Stashes your files into ".yellow(),
        env!("HOME").yellow(),
        "/.config/".yellow(),
        "unstash".magenta(),
        "->".red(),
        "unstashed your symbolic links from ".yellow(),
        env!("HOME").yellow(),
        "/.config/".yellow()
    );
    help
}
