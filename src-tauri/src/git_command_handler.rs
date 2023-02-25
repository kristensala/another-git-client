use std::process::Command;
use std::str;

use anyhow::Result;
use tauri::State;

use crate::app::{App, AppState};
use crate::command_line_parser::Commit;

#[tauri::command]
pub fn git_log_command() {
    // TODO: get changed files and the changed contents
    let mut command = Command::new("git");
    command.arg("log");

    let output = command.output().expect("git log command failed");
    let output_utf8 = str::from_utf8(&output.stdout).unwrap();
    let commits: Vec<&str> = output_utf8.split("commit ").collect();

    let commits_parsed = commits.iter()
        .map(|x| x.parse::<Commit>().unwrap())
        .collect::<Vec<Commit>>();
    
    println!("{:#?}", commits_parsed);
}

//https://github.com/tauri-apps/tauri/discussions/1336
#[tauri::command]
pub fn git_branch_command() -> Vec<String> {
    let mut command = Command::new("git");
    command.arg("branch");

    let output = command.output().expect("git branch command failed");
    let output_utf8 = str::from_utf8(&output.stdout).unwrap();
    
    let local_branches: Vec<String> = output_utf8.lines()
        .map(|x| x.to_string())
        .collect();

    return local_branches;
}

pub fn git_new_branch_command(branch_name: &str) {
    todo!("create a new branch and checkout");
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

#[tauri::command]
pub fn git_stash_command(files: Vec<&str>) {
    todo!("if files empty stash all");
}

#[tauri::command]
pub fn git_diff_command(file_path: &str) {
    if file_path.is_empty() {
        let mut command = Command::new("git");
        command.arg("diff");

        return;
    }

    let diff = Command::new("git")
        .arg("diff")
        .arg(file_path);

    todo!("if file param is empty then take all the file diffs");
}
