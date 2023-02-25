use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct AppState {
    pub current_dir: Option<String>,
}

#[derive(Debug)]
pub struct App(pub Mutex<AppState>);

impl AppState {
    pub fn init() -> AppState {
        return AppState {
            current_dir: None, // TODO: if repo saved, take the first one
        };
    }
}

