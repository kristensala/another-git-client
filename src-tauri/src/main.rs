#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::process::Command;
use std::str;

use git_commands::Commit;

pub mod git_commands;
pub mod app;
pub mod repository;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ping])
        .invoke_handler(tauri::generate_handler![test_command_line])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn ping() -> String {
    return String::from("Pong");
}

#[tauri::command]
fn test_command_line() {
    let mut git_log = Command::new("git");
    git_log.arg("log");

    let res = git_log.output().expect("git log command failed");
    let out = str::from_utf8(&res.stdout).unwrap();
    let test: Vec<&str> = out.split("commit ").collect();

    let commits = test.iter()
        .map(|x| x.parse::<Commit>().unwrap())
        .collect::<Vec<Commit>>();


    // here i can impl Fromiter for git log and parse the lines
    
    println!("{:#?}", commits);
}
