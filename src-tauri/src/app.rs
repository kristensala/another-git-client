use std::sync::Mutex;

use anyhow::{Result, anyhow};

use crate::repository::Repository;

#[derive(Debug, Clone)]
pub struct AppState {
    pub selected_repo: Option<Repository>, // this should be selected repository
    pub repositories: Vec<Repository>
}

#[derive(Debug)]
pub struct App(pub Mutex<AppState>);

impl AppState {
    pub fn init() -> AppState {
        return AppState {
            selected_repo: None, // TODO: if repo saved, take the first one
            repositories: vec![]
        };
    }

    pub fn create_repository(&mut self, name: String, path: String) -> Result<(), anyhow::Error> {
        let create = Repository::create(name, path);

        if create.is_ok() {
            self.repositories.push(create.unwrap());
            return Ok(());
        }

        return Err(anyhow!("Creating repositorie failed!"));
    }

    pub fn get_repository(&self, path: &str) -> Repository {
        let repo = Repository::get(path);
        return repo;
    }

    pub fn is_valid_dir(&self, selected_dir: &str) -> bool {
        // if dir does not contain .git folder return false
        todo!();
    }
}

