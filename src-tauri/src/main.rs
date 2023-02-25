#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use app::{AppState, App};


pub mod command_line_parser;
pub mod git_command_handler;
pub mod app;
pub mod app_command_handler;

#[tauri::command]
fn ping() -> String {
    return String::from("Pong");
}

fn main() {
    tauri::Builder::default()
        .manage(App(Mutex::new(AppState::init())))
        .invoke_handler(tauri::generate_handler![ping])
        .invoke_handler(tauri::generate_handler![git_command_handler::git_log_command])
        .invoke_handler(tauri::generate_handler![git_command_handler::git_branch_command])
        .invoke_handler(tauri::generate_handler![app_command_handler::get_current_working_dir, app_command_handler::change_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

