use simplelog::{ Config as LogConfig, LevelFilter, WriteLogger };
use std::fs::OpenOptions;
use std::process;

/// Initialisiert den Logger und schreibt Logs in die Datei `application.log`.
pub fn init_logger() {
    let log_file = OpenOptions::new()
        .create(true) // Erstelle die Datei, falls sie nicht existiert
        .append(true) // Füge an die bestehende Datei an
        .open("application.log")
        .unwrap_or_else(|e| {
            log::error!("Failed to open log file: {}", e);
            process::exit(1);
        });

    WriteLogger::init(LevelFilter::Info, LogConfig::default(), log_file).unwrap_or_else(|e| {
        eprintln!("Failed to initialize logger: {}", e);
        process::exit(1);
    });

    log::info!("Logger initialized.");
}
