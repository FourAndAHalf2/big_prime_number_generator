use std::sync::{LazyLock, Mutex};

pub struct Settings{
    pub show_bar: bool
}

impl Settings {
    pub fn new() -> Settings{
        return Settings { show_bar: true };
    }
}

pub static SETTINGS: LazyLock<Mutex<Settings>> = LazyLock::new(|| Settings::new().into());

pub fn get_settings() -> std::sync::MutexGuard<'static, Settings> {
    return SETTINGS.lock().unwrap();
}

