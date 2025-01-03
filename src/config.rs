use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub source_folder: String,
    pub destination_folder: String,
    pub sync_interval_seconds: u64,
}

pub fn load_config(file_path: &str) -> Config {
    let config_data = fs::read_to_string(file_path).expect("Unable to read config file");
    serde_json::from_str(&config_data).expect("JSON was not well-formatted")
}
