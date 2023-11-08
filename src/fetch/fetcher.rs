use crate::linker::ToPathbuf;
use crate::{error, success};
use owo_colors::OwoColorize;

use super::Fetch;
use git2::build::RepoBuilder;

impl Fetch {
    pub fn new(url: String, branch: Option<String>) -> Self {
        Self { url, branch }
    }
    pub fn clone(&self) {
        let dot_dir = "dotfiles".home_path();
        let mut builder = RepoBuilder::new();

        if self.branch.is_some() {
            builder.branch(&self.branch.clone().unwrap());
        }

        if dot_dir.exists() {
            error!("Fatal: {} exists!", &dot_dir.display());
            return;
        }
        if let Err(e) = builder.clone(&self.url, &dot_dir) {
            error!("Fatal: Failed to clone specified dotfile repo: {}", e);
            return;
        }

        success!(
            "succesfully cloned dotfiles repo into {}",
            dot_dir.display()
        );
    }
}
