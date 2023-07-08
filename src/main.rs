#![feature(let_chains)]
use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use std::{env, fs, os::unix::fs::symlink};

const DEST_DIR: &str = "./config";
const SOURCE_DIR: &str = "./dir";
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    subcommand: SubCommand,
}
#[derive(Subcommand, Debug)]
enum SubCommand {
    Stash,
    Unstash,
}
fn main() {
    let args = Args::parse();
    match args.subcommand {
        SubCommand::Stash => linker(),
        SubCommand::Unstash => {
            if let Err(e) = remove_symbolic_links_in_directory(DEST_DIR) {
                println!("Something went wrong when removing symbolic links");
                if let Ok(rust_backtrace) = env::var("RUST_BACKTRACE") && rust_backtrace == "1" {
                    println!("{:?}", e);
                }
            }
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
            Err(_) => {
                println!("Files may already exist");
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
