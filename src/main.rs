use std::path::Path;
use std::sync::{ atomic::{ AtomicBool, Ordering }, Arc };
use std::thread;
use std::time::Duration;
use std::fs::OpenOptions;
use std::process;
use config::load_config;
use sync::sync_folders;
use input::start_input_listener;
use simplelog::{ Config as LogConfig, LevelFilter, WriteLogger };

mod config;
mod sync;
mod input;

fn main() {
    // Initialisiere Logging
    init_logger();

    log::info!("Application started.");

    // Lade die Konfiguration
    let config = load_config("config.json");

    // Pfade definieren
    let source_path = Path::new(&config.source_folder);
    let destination_path = Path::new(&config.destination_folder);

    // Überprüfe, ob die Ordner existieren
    if !source_path.exists() {
        log::error!("Source folder does not exist: {}", config.source_folder);
        return;
    }
    if !destination_path.exists() {
        std::fs::create_dir_all(destination_path).expect("Failed to create destination folder");
    }

    // Gemeinsamer Zustand für die Beendigung
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    // Separater Thread für Benutzereingaben
    thread::spawn(move || start_input_listener(running_clone));

    // Hauptschleife zur Synchronisation
    while running.load(Ordering::Relaxed) {
        log::info!("Starting synchronization...");
        let copied_files = sync_folders(source_path, destination_path);
        log::info!("Synchronization complete. {} files were copied.", copied_files);

        // Warte das konfigurierte Intervall ab
        let wait_time = Duration::from_secs(config.sync_interval_seconds);
        log::info!(
            "Waiting for {} seconds before the next synchronization...",
            config.sync_interval_seconds
        );

        let mut elapsed = 0;
        while elapsed < wait_time.as_secs() && running.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(1));
            elapsed += 1;
        }
    }

    log::info!("Program terminated.");
}

fn init_logger() {
    let log_file = OpenOptions::new()
        .create(true) // Erstelle die Datei, falls sie nicht existiert
        .append(true) // Füge an die bestehende Datei an
        .open("application.log")
        .unwrap_or_else(|e| {
            eprintln!("Failed to open log file: {}", e);
            process::exit(1);
        });

    WriteLogger::init(LevelFilter::Info, LogConfig::default(), log_file).unwrap_or_else(|e| {
        eprintln!("Failed to initialize logger: {}", e);
        process::exit(1);
    });

    log::info!("Logger initialized.");
}
