use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Repository {
    pub name: String,
    pub path: String
}

impl Repository {
    pub fn create(name: String, path: String) -> Result<Repository> {
        todo!();
    }

    pub fn get(path: &str) -> Repository {
        todo!();
    }

    fn is_valid_repository(path: String) -> bool {
        todo!();
    }
}
