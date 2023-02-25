use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct App {
    pub current_dir: String
}

#[derive(Debug)]
pub struct AppState(pub Mutex<App>);

impl App {
    pub fn init() -> App {
        return App {
            current_dir: String::from("test")
        };
    }
}

