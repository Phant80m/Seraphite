use {
    super::{Linker, ToPathbuf},
    crate::{error, success, warning},
    anyhow::Result,
    owo_colors::OwoColorize,
    std::{
        fs, io,
        os::unix::fs::symlink,
        path::{Path, PathBuf},
    },
};

fn copy_directory(source: &Path, dest: &Path) -> Result<()> {
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
fn create_backup(source_dir: &Path, backup_dir: &Path) -> Result<()> {
    fs::create_dir_all(backup_dir)?;

    let current_datetime = chrono::Local::now();
    let backup_folder_name = format!("backup-{}", current_datetime.format("%Y-%m-%d_%H-%M-%S"));
    let dest_dir = backup_dir.join("backup").join(backup_folder_name);

    copy_directory(source_dir, &dest_dir)?;

    success!("Backup created!");

    Ok(())
}
impl Linker {
    pub fn parse(i: PathBuf, d: PathBuf) -> Self {
        Self {
            input: i,
            destination: d,
        }
    }
    pub fn create_link(&self) -> Result<()> {
        let path = fs::read_dir(&self.input)?;
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
                "y" => create_backup(source_dir, &backup_dir)?,
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
                "y" => create_backup(source_dir, &backup_dir)?,
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
            let entry = entry?.path().canonicalize().unwrap();
            let entry_filename = entry.file_name().unwrap();
            let destination_entry = self.destination.join(entry_filename);
            let pretty = format!("~/.config/{}", entry_filename.to_str().unwrap());
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
        if !self.destination.exists() {
            return Ok(());
        }
        for entry in fs::read_dir(&self.destination)? {
            let entry = entry?;
            match entry.path().symlink_metadata() {
                Ok(metadata) => {
                    if metadata.file_type().is_symlink() {
                        fs::remove_file(&entry.path())?;
                        success!("Removed symbolic link at: {}", entry.path().display());
                    }
                }
                Err(_) => {
                    error!(
                        "Failed to get symlink metadata for {}",
                        entry.path().display()
                    );
                }
            }
        }
        Ok(())
    }
}
