use std::ffi::OsString;

use {
    super::{Linker, ToPathbuf},
    crate::{error, success, warning},
    anyhow::Result,
    owo_colors::OwoColorize,
    std::{
        collections::HashSet,
        fs, io,
        os::unix::fs::symlink,
        path::{Path, PathBuf},
    },
};

impl Linker {
    pub fn new(i: PathBuf, d: PathBuf) -> Self {
        let to_hashset = |path: &PathBuf| -> HashSet<OsString> {
            path.read_dir()
                .unwrap_or_else(|e| {
                    error!("Failed to read directory {}: {}", path.display(), e);
                    std::process::exit(0)
                })
                .filter_map(|entry| {
                    entry
                        .ok()
                        .map(|dir| dir.path().file_name().unwrap().to_owned())
                })
                .collect()
        };
        let input_inodes = to_hashset(&i);
        let dest_inodes = to_hashset(&d);

        Self {
            input: i,
            destination: d,
            input_inodes,
            dest_inodes,
        }
    }
    pub fn create_link(&self) -> Result<()> {
        let path = match fs::read_dir(&self.input) {
            Ok(read_dir) => read_dir,
            Err(e) => {
                error!("Error finding .config in the dotfile directory");
                return Err(e.into());
            }
        };
        let backup_dir = ".seraphite".home_path();
        let source = format!("{}/.config", std::env::var("HOME").unwrap());
        let source_dir = Path::new(&source);

        if backup_dir.exists() {
            warning!("would you like to create a backup? y / n");
            let mut backup: String = String::new();
            io::stdin()
                .read_line(&mut backup)
                .expect("failed to read user line!");
            match backup.trim() {
                "y" => Self::create_backup(source_dir, &backup_dir)?,
                "n" => {
                    warning!("Proceeding without backup!");
                }
                _ => {
                    error!("input was not y / no!");
                    std::process::exit(0)
                }
            }
        } else {
            warning!("Backup does not exist! would you like to create one? y / n");
            let mut backup: String = String::new();
            io::stdin()
                .read_line(&mut backup)
                .expect("failed to read user line!");
            match backup.trim() {
                "y" => Self::create_backup(source_dir, &backup_dir)?,
                "n" => {
                    warning!("Proceeding without backup!");
                }
                _ => {
                    error!("input was not y / no!");
                    std::process::exit(0)
                }
            }
        }
        for entry in path {
            // remove unwrap later on:
            let entry = entry?.path().canonicalize()?;
            let entry_filename = entry.file_name().unwrap();
            let destination_entry = self.destination.join(entry_filename);
            let pretty = format!(
                "~/.config/{}",
                entry_filename.to_str().expect(&format!(
                    "failed to decode path into utf8: {}",
                    entry.to_string_lossy()
                ))
            );
            if let Err(e) = symlink(&entry, &destination_entry) {
                error!(
                    "failed to link files into: {} {} {:?}",
                    pretty,
                    "->".cyan().bold(),
                    e.bold().red()
                );
                if destination_entry.exists() {
                    if !&destination_entry.is_dir() {
                        warning!("Overriden: {}", &destination_entry.display());
                        fs::remove_file(&destination_entry)?;
                    } else {
                        warning!(
                            "Overriden: {}, attempting to link again!",
                            &destination_entry.display()
                        );
                        fs::remove_dir_all(&destination_entry)?;
                    }
                }
                if let Err(e) = symlink(&entry, &destination_entry) {
                    error!(
                        "failed to link files into: {} {} {:?}",
                        pretty,
                        "->".cyan().bold(),
                        e.bold().red()
                    );
                }
            } else {
                success!(
                    "Linked: {} {} {}",
                    entry.clone().display(),
                    "->".cyan().bold(),
                    destination_entry.clone().display()
                );
            }
        }
        Ok(())
    }
    pub fn remove_link(&self) -> Result<()> {
        for name in self.dest_inodes.intersection(&self.input_inodes) {
            let config_link = self.destination.join(name);
            if !config_link.exists() {
                continue;
            }
            if !config_link.is_symlink() {
                error!("refusing to delete a non Symlink");
            }
            match config_link.symlink_metadata() {
                Ok(metadata) => {
                    if metadata.file_type().is_symlink() {
                        fs::remove_file(&config_link)?;
                        success!("Removed symbolic link at: {}", config_link.display());
                    }
                }
                Err(_) => {
                    error!(
                        "Failed to get symlink metadata for {}",
                        config_link.display()
                    );
                }
            }
        }
        Ok(())
    }
    fn copy_directory(source: &Path, dest: &Path) -> Result<()> {
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let source_path = entry.path();
            let dest_path = dest.join(source_path.file_name().unwrap());

            if source_path.is_file() {
                fs::copy(&source_path, &dest_path)?;
            } else if source_path.is_dir() {
                fs::create_dir_all(&dest_path)?;
                Self::copy_directory(&source_path, &dest_path)?;
            }
        }

        Ok(())
    }
    fn create_backup(source: &Path, backup: &Path) -> Result<()> {
        fs::create_dir_all(backup)?;

        let current_datetime = chrono::Local::now();
        let backup_folder_name = format!("backup-{}", current_datetime.format("%Y-%m-%d_%H-%M-%S"));
        let dest_dir = backup.join("backup").join(backup_folder_name);

        Self::copy_directory(source, &dest_dir)?;

        success!("Backup created!");

        Ok(())
    }
}
