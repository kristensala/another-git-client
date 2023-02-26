use std::process::Command;
use std::str;

use anyhow::{Result, Ok};
use tauri::State;

use crate::{command_line_parser::{Commit, Branch}, app::App};

#[tauri::command]
pub fn git_log_command(app: State<App>) -> Vec<Commit> {
    // TODO: get changed files and the changed contents
    let mut command = Command::new("git");
    command.arg("log");

    let output = command.output().expect("git log command failed");
    let output_utf8 = str::from_utf8(&output.stdout).unwrap();
    let commits: Vec<&str> = output_utf8.split("commit ").collect();

    let commits_parsed = commits.iter()
        .map(|x| x.parse::<Commit>().unwrap())
        .collect::<Vec<Commit>>();
    
    return commits_parsed;
}

//https://github.com/tauri-apps/tauri/discussions/1336
#[tauri::command]
pub fn git_branch_command(app: State<App>) -> Vec<Branch> {
    let mut command = Command::new("git");
    command.arg("branch");

    let output = command.output().expect("git branch command failed");
    let output_utf8 = str::from_utf8(&output.stdout).unwrap();
    
    let local_branches: Vec<Branch> = output_utf8.lines()
        .map(|x| x.parse::<Branch>().unwrap())
        .collect();

    return local_branches;
}

pub fn git_new_branch_command(app: State<App>, branch_name: &str) {
    todo!("create a new branch and checkout");
}

#[tauri::command]
pub fn git_checkout_command(app: State<App>, target_branch: &str) -> Result<()> {
    let state_guard = app.0.lock().unwrap();

    let mut command = Command::new("git");
    command.arg("checkout");
    command.arg(target_branch);

    command.current_dir(state_guard.current_dir.unwrap());
    let output = command.output().expect("checkout failed");

    // Possible errors
    // * branch does not exist
    // * invalid working dir
    // * uncommited changes
    //

    return Ok(());
}

#[tauri::command]
pub fn git_commit_command(app: State<App>, message: &str) -> Result<()> {
    todo!();
}

#[tauri::command]
pub fn git_push_command(app: State<App>, branch: &str) -> Result<()> {
    todo!();
}

#[tauri::command]
pub fn git_stash_command(app: State<App>, files: Vec<&str>) {
    todo!("if files empty stash all");
}

#[tauri::command]
pub fn git_diff_command(app: State<App>, file_path: &str) {
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
