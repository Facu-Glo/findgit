use crate::models::Config;
use std::path::PathBuf;

fn get_config_path(path: &str) -> PathBuf {
    PathBuf::from(shellexpand::tilde(path).into_owned())
}

pub fn load_config(path: &str) -> Config {
    let dir = get_config_path(path);
    let config_data = std::fs::read_to_string(dir).expect("Could not read config.json");

    serde_json::from_str(&config_data).expect("Invalid JSON format")
}
