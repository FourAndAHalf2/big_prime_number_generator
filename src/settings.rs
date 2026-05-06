use std::{
    env,
    fs::File,
    sync::{LazyLock, Mutex},
};

use crate::settings;

pub struct Settings {
    pub output: String,
    pub show_bar: bool,
    pub buffer_size: usize, //number of bytes
    pub sieve_type: String,
    pub io_method: String,
}

impl Settings {
    pub fn new() -> Settings {
        return Settings {
            output: "out.txt".to_string(),
            show_bar: false,
            buffer_size: 1_000,
            sieve_type: "eratosthenes".to_string(),
            io_method: "text".to_string(),
        };
    }
}

pub static SETTINGS: LazyLock<Mutex<Settings>> = LazyLock::new(|| Settings::new().into());

pub fn get_settings() -> std::sync::MutexGuard<'static, Settings> {

    return SETTINGS.lock().unwrap();
}

pub fn load_settings() -> Result<(), Box<dyn std::error::Error>> {
    let mut config_path = env::current_exe()?;

    config_path.pop();
    config_path.pop();
    config_path.pop();
    config_path.push("config.json");

    let config_file = File::open(config_path)?;

    let json: serde_json::Value = serde_json::from_reader(config_file)?;

    // return  Ok(());

    
    //it is the easiest option to ensure that code won't crash
    get_settings().buffer_size = json["buffer_size"]
        .as_number()
        .unwrap()
        .as_u64()
        .unwrap_or(Settings::new().buffer_size as u64) as usize; // there can be errors in 32 bits systems

   
    get_settings().show_bar = json["show_bar"]
        .as_bool()
        .unwrap_or(Settings::new().show_bar);

  
    get_settings().output = json["output"]
        .as_str()
        .unwrap_or(&Settings::new().output)
        .to_string();
    get_settings().sieve_type = json["sieve_type"]
        .as_str()
        .unwrap_or(&Settings::new().sieve_type)
        .to_string();
    get_settings().io_method = json["method"]
        .as_str()
        .unwrap_or(&Settings::new().io_method)
        .to_string();

    Ok(())
}

pub fn load_and_get_settings() -> std::sync::MutexGuard<'static, Settings> {
    let _ = load_settings();
    get_settings()
}
