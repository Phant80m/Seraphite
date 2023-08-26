use super::{Linker, ToPathbuf};
use owo_colors::OwoColorize;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::{fs, io};
fn copy_directory(source: &Path, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let dest_path = dest.join(source_path.file_name().unwrap());

        if source_path.is_file() {
            fs::copy(&source_path, &dest_path)?;
        } else if source_path.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_directory(&source_path, &dest_path)?;
        }
    }

    Ok(())
}
impl Linker {
    pub fn parse(i: PathBuf, d: PathBuf) -> Self {
        Self {
            input: i,
            destination: d,
        }
    }
    pub fn create_link(&self) -> io::Result<()> {
        let path = fs::read_dir(&self.input)?;
        let backup_dir = ".seraphite".home_path();
        if backup_dir.exists() { /* to be added */
        } else {
            println!(
                "{} Backup does not exist! would you like to create one? y / n",
                "[  ]".yellow().bold()
            );
            let mut backup: String = String::new();
            io::stdin()
                .read_line(&mut backup)
                .expect("failed to read user line!");
            match backup.trim() {
                "y" => {
                    fs::create_dir(&backup_dir)?;
                    let source = format!("{}/.config", std::env::var("HOME").unwrap());
                    let source_dir = Path::new(&source);
                    let dest_dir = PathBuf::from(&backup_dir.join("backup"));

                    for entry in fs::read_dir(source_dir)? {
                        let entry = entry?;
                        let source_path = entry.path();

                        let dest_path = dest_dir.join(source_path.file_name().unwrap());

                        if source_path.is_file() {
                            fs::copy(&source_path, &dest_path)?;
                        } else if source_path.is_dir() {
                            fs::create_dir_all(&dest_path)?;
                            copy_directory(&source_path, &dest_path).unwrap();
                        }
                    }
                    println!("{} Backup created!", "[  ]".green().bold(),);
                }
                "n" => {
                    println!("{} Proceeding without backup!", "[  ]".yellow().bold());
                }
                _ => println!("{} input was not y / no!", "[  ]".red().bold(),),
            }
        }
        for entry in path {
            // remove unwrap later on:
            let entry = entry?.path().canonicalize().unwrap();
            let entry_filename = entry.file_name().unwrap();
            let destination_entry = self.destination.join(entry_filename);
            let pretty = format!("~/.config/{}", entry_filename.to_str().unwrap());
            if let Err(e) = symlink(&entry, &destination_entry) {
                println!(
                    "{} failed to link files into: {} {} {:?}",
                    "[  ]".red().bold(),
                    pretty,
                    "->".cyan().bold(),
                    e.bold().red()
                );
                if destination_entry.exists() {
                    if !&destination_entry.is_dir() {
                        print!(
                            "{} Overriden: {}",
                            "[  ]".yellow().bold(),
                            &destination_entry.display()
                        );
                        fs::remove_file(&destination_entry)?;
                    } else {
                        print!(
                            "{} Overriden: {}",
                            "[  ]".yellow().bold(),
                            &destination_entry.display()
                        );
                        fs::remove_dir_all(&destination_entry)?;
                    }
                }
                println!(", attempting to link again!");
                if let Err(e) = symlink(&entry, &destination_entry) {
                    println!(
                        "{} failed to link files into: {} {} {:?}",
                        "[  ]".red().bold(),
                        pretty,
                        "->".cyan().bold(),
                        e.bold().red()
                    );
                }
            } else {
                println!(
                    "{} Linked: {} {} {}",
                    "[  ]".green().bold(),
                    entry.clone().display(),
                    "->".cyan().bold(),
                    destination_entry.clone().display()
                );
            }
        }
        Ok(())
    }
    pub fn remove_link(&self) -> io::Result<()> {
        if !self.destination.exists() {
            return Ok(());
        }
        for entry in fs::read_dir(&self.destination)? {
            let entry = entry?;
            match entry.path().symlink_metadata() {
                Ok(metadata) => {
                    if metadata.file_type().is_symlink() {
                        fs::remove_file(&entry.path())?;
                        println!(
                            "{} Removed symbolic link at: {}",
                            "[  ]".green().bold(),
                            entry.path().display()
                        );
                    }
                }
                Err(_) => {
                    println!(
                        "Failed to get symlink metadata for {}",
                        entry.path().display()
                    );
                }
            }
        }
        Ok(())
    }
}
