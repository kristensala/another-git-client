use std::process::Command;
use std::str;

use anyhow::Result;

use crate::command_line_parser::Commit;

#[tauri::command]
pub fn git_log_command() {
    // TODO: get changed files and the changed contents
    let mut git_log = Command::new("git").arg("log");

    let output = git_log.output().expect("git log command failed");
    let output_utf8 = str::from_utf8(&output.stdout).unwrap();
    let commits: Vec<&str> = output_utf8.split("commit ").collect();

    let commits_parsed = commits.iter()
        .map(|x| x.parse::<Commit>().unwrap())
        .collect::<Vec<Commit>>();
    
    println!("{:#?}", commits_parsed);
}

#[tauri::command]
pub fn git_branch_command() -> Vec<&'static str> {
    let mut git_branch = Command::new("git").arg("branch");

    let output = git_branch.output().expect("git branch command failed");
    let output_utf8 = str::from_utf8(&output.stdout).unwrap();
    
    let local_branches: Vec<&str> = output_utf8.lines().collect();
    return local_branches;
}

#[tauri::command]
pub fn git_checkout_command(target_branch: &str) -> Result<()> {
    todo!();
}

#[tauri::command]
pub fn git_commit_command(message: &str) -> Result<()> {
    todo!();
}

#[tauri::command]
pub fn git_push_command(branch: &str) -> Result<()> {
    todo!();
}
