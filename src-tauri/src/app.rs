use anyhow::Result;

use crate::repository::Repository;

pub enum ColorMode {
    Dark,
    Light
}

pub struct Settings {
    pub color: ColorMode
}

// here is application logic
// saved repositories
// settings
// git flow settings
// etc.
pub struct App {
    pub repositories: Vec<Repository>,
    pub selected_repository: Repository,
}

impl App {
    pub fn load_repositories() -> Result<Vec<Repository>> {
        todo!();
    }

    // click on a tab to go to another repo view
    pub fn switch_repository(&mut self, repo: Repository) {
        self.selected_repository = repo;
    }
}
