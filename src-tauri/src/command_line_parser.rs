use std::str::FromStr;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Branch {
    pub name: String,
    pub is_active: bool
}

#[derive(Debug)]
pub struct ParseBranchError;

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

impl FromStr for Branch {
    type Err = ParseBranchError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if line.starts_with("*") {
            let split_line = line.split("*").collect::<Vec<&str>>();
            return Ok(Branch {
                name: split_line[1].trim().to_string(),
                is_active: true
            });
        }

        return Ok(Branch {
            name: line.to_string(),
            is_active: false
        });
    }
}

