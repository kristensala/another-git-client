#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


pub mod command_line_parser;
pub mod git_command_handler;
pub mod app;
pub mod repository;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ping])
        .invoke_handler(tauri::generate_handler![git_command_handler::git_log_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn ping() -> String {
    return String::from("Pong");
}

