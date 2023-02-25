use tauri::State;

use crate::{app::AppState, repository::Repository};

#[tauri::command]
pub fn change_dir(app: State<AppState>, new_dir: String) {
    let mut state_guard = app.0.lock().unwrap();

    let mut app_state = state_guard;
    app_state.current_dir = new_dir;
}

#[tauri::command]
pub fn get_current_working_dir(app: State<AppState>) -> String {
    let mut state_guard = app.0.lock().unwrap();

    let mut app_state = state_guard;
    return app_state.current_dir.to_owned();
}
