use std::fs;
use crate::models::Config;

pub fn load_config() -> Config {
    let config_path = shellexpand::tilde("~/.config/findgit/config.json").into_owned();
    let config_data = fs::read_to_string(config_path)
        .expect("No se pudo leer config.json");
    
    serde_json::from_str(&config_data)
        .expect("Error en formato JSON")
}
