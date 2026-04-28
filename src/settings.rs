use std::sync::{LazyLock, Mutex};

pub struct Settings{
    pub show_bar: bool,
    pub buffor_size: usize//number of bytes
}

impl Settings {
    pub fn new() -> Settings{
        return Settings { show_bar: false, buffor_size: 1_000};
    }
}

pub static SETTINGS: LazyLock<Mutex<Settings>> = LazyLock::new(|| Settings::new().into());

pub fn get_settings() -> std::sync::MutexGuard<'static, Settings> {
    return SETTINGS.lock().unwrap();
}

