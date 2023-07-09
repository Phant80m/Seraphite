#![feature(let_chains)]
use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use seraphite::help;
use std::{env, fs, os::unix::fs::symlink};
const DEST_DIR: &str = "./config";
const SOURCE_DIR: &str = "./dir";
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, disable_help_flag = true, disable_help_subcommand = true)]
#[command(arg_required_else_help = false)]
struct Args {
    #[command(subcommand)]
    subcommand: Option<SubCommand>,
}
#[derive(Subcommand, Debug)]
enum SubCommand {
    Help,
    Stash,
    Unstash,
}
fn main() {
    let help = help();
    let args = Args::parse();
    match args.subcommand {
        Some(SubCommand::Stash) => linker(),
        Some(SubCommand::Unstash) => {
            if let Err(e) = remove_symbolic_links_in_directory(DEST_DIR) {
                println!("Something went wrong when removing symbolic links");
                if let Ok(rust_backtrace) = env::var("RUST_BACKTRACE") && rust_backtrace == "1" {
                    println!("{:?}", e);
                }
            }
        }
        Some(SubCommand::Help) => {
            println!("{help}");
        }
        None => {
            println!("{help}");
        }
    }
}
fn linker() {
    let source_dir = SOURCE_DIR;
    let dest_dir = DEST_DIR;

    fs::create_dir_all(dest_dir).unwrap();

    for entry in fs::read_dir(source_dir).unwrap() {
        let source_path = entry.unwrap().path().canonicalize().unwrap();
        let file_name = source_path.file_name().unwrap();
        let dest_file = format!("{}/{}", dest_dir, file_name.to_string_lossy());

        match symlink(source_path, dest_file.clone()) {
            Ok(push) => push,
            Err(e) => {
                println!("Files may already exist");
                if let Ok(rust_backtrace) = env::var("RUST_BACKTRACE") && rust_backtrace == "1" {
                    println!("{:?}", e);
                }
                break;
            }
        }
    }
}
fn remove_symbolic_links_in_directory(directory: &str) -> Result<(), std::io::Error> {
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Ok(metadata) = fs::symlink_metadata(&path) && metadata.file_type().is_symlink() && let Err(_) = fs::remove_file(&path) {
                // println!("Error removing symbolic link: {}", err);
                println!("{}", "Failed to remove symbolic links!".red())
            }
        }
    }
    println!("{}", "Successfully removed symbolic links!".green());
    Ok(())
}
