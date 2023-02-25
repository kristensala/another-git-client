use tauri::State;

use crate::app::App;

#[tauri::command]
pub fn change_dir(app: State<App>, new_dir: String) {
    let mut state_guard = app.0.lock().unwrap();
    state_guard.current_dir = Some(new_dir);
}

#[tauri::command]
pub fn get_current_working_dir(app: State<App>) -> Option<String> {
    let state_guard = app.0.lock().unwrap();
    return state_guard.current_dir.to_owned();
}
