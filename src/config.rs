use serde::Deserialize;
use std::fs;

/// Struktur für die Konfiguration
#[derive(Deserialize)]
pub struct Config {
    pub source_folder: String,
    pub destination_folder: String,
    pub sync_interval_seconds: u64,
}

/// Funktion zum Laden der Konfiguration aus einer JSON-Datei
pub fn load_config(file_path: &str) -> Config {
    let config_data = fs::read_to_string(file_path).expect("Unable to read config file");
    serde_json::from_str(&config_data).expect("JSON was not well-formatted")
}
