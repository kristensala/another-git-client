use tauri::State;

use crate::{app::App, repository::Repository};

#[tauri::command]
pub fn change_dir(app: State<App>, new_dir: String) -> Result<(), String> {
    let mut state_guard = app.0.lock().unwrap();
    
    if state_guard.is_valid_dir(&new_dir) {
        let new_repo = state_guard.get_repository(&new_dir);
        state_guard.selected_repo = Some(new_repo);
        return Ok(());
    }

    return Err("Not a valid directory!".into());
}

#[tauri::command]
pub fn get_current_repository(app: State<App>) -> Option<Repository> {
    let state_guard = app.0.lock().unwrap();
    return state_guard.selected_repo.to_owned();
}


