use anyhow::Result;

pub struct Repository {
    pub name: String,
    pub path: String, // unique
    pub working_branch: String
}

impl Repository {
    pub fn create() -> Result<Repository> {
        todo!("is repo exists in memory dont allow to make another one");
    }

    pub fn get_working_branch() -> String {
        todo!();
    }
}
