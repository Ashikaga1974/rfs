use std::path::Path;
use std::sync::{ atomic::{ AtomicBool, Ordering }, Arc };
use std::thread;
use std::time::Duration;
use config::load_config;
use sync::sync_folders;
use device_query::{ DeviceQuery, DeviceState, Keycode };
use initlogger::init_logger;

mod config;
mod sync;
mod initlogger;

fn main() {
    // Initialisiere Logging
    init_logger();

    log::info!("Application started.");

    let config_file_path = "config.json";

    if !Path::new(config_file_path).exists() {
        log::error!("Configuration file '{}' does not exist.", config_file_path);
        std::process::exit(1);
    }

    log::info!("Configuration file '{}' found. Proceeding with initialization.", config_file_path);

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

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running); // Klon für den Thread

    // Starte den Thread für Tastatureingaben
    let _handle = thread::spawn(move || {
        let device_state = DeviceState::new();
        while running_clone.load(Ordering::Relaxed) {
            let keys: Vec<Keycode> = device_state.get_keys();
            if keys.contains(&Keycode::LAlt) && keys.contains(&Keycode::A) {
                log::info!("'ALT+a' pressed!");
                running_clone.store(false, Ordering::Relaxed); // Verwende den Klon
                log::info!("Exiting...");
                break;
            }
            thread::sleep(Duration::from_millis(100)); // Vermeide hohe CPU-Auslastung
        }
    });

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

// fn init_logger() {
//     let log_file = OpenOptions::new()
//         .create(true) // Erstelle die Datei, falls sie nicht existiert
//         .append(true) // Füge an die bestehende Datei an
//         .open("application.log")
//         .unwrap_or_else(|e| {
//             eprintln!("Failed to open log file: {}", e);
//             process::exit(1);
//         });

//     WriteLogger::init(LevelFilter::Info, LogConfig::default(), log_file).unwrap_or_else(|e| {
//         eprintln!("Failed to initialize logger: {}", e);
//         process::exit(1);
//     });

//     log::info!("Logger initialized.");
// }
