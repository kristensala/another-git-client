use std::{str::FromStr};

pub enum GitCommand {
    Log,
    Merge,
    Pull,
    Push,
    Commit,
    Checkout,
    Branch
}

pub struct GitAction {
    command: GitCommand,
    working_branch: String,
    target_branch: String
}

#[derive(Debug)]
pub struct Commit {
    pub author: Option<String>,
    pub description: Option<String>,
    pub date: Option<String>,
}

impl Commit {
    fn new() -> Commit {
        return Commit {
            author: None,
            description: None,
            date: None
        };
    }
}

#[derive(Debug)]
pub struct ParseCommitError;

// TODO: this looks horrible
impl FromStr for Commit {
    type Err = ParseCommitError;

    fn from_str(s: &str) -> Result<Commit, Self::Err> {
        let commit_line: Vec<&str> = s.split("\n").collect();
        let desc_line: Vec<&str> = s.split("\n\n").collect();

        let mut commit = Commit::new();

        if desc_line.len() > 1 {
            commit.description = Some(desc_line[1].trim().to_string());
        }

        for line in commit_line {
            if line.is_empty() {
                continue;
            }

            if line.starts_with("Author:") {
                let author_line = line.split(":").collect::<Vec<&str>>();
                commit.author = Some(author_line[1].trim().to_string());
            }

            if line.starts_with("Date:") {
                let date_line = line.split(":").collect::<Vec<&str>>();
                commit.date = Some(date_line[1].trim().to_string());
            }
        }
        
        return Ok(commit);
    }
}

