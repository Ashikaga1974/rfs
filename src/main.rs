#![windows_subsystem = "windows"]

use std::path::Path;
use std::sync::{ atomic::{ AtomicBool, Ordering }, Arc };
use std::thread;
use std::time::Duration;
use config::load_config;
use sync::sync_folders;
use device_query::{ DeviceQuery, DeviceState, Keycode };
use initlogger::init_logger;
use notification::{ send_start_notification, send_custom_notification };
use tray_item::{ IconSource, TrayItem };

mod config;
mod sync;
mod initlogger;
mod notification;

const CONFIG_FILE_PATH: &str = "config.json";

fn main() {
    let mut _tray = TrayItem::new(
        "Beenden mit ALT+a",
        IconSource::Resource("aa-exe-icon")
    ).unwrap();

    // Initialisiere Logging
    init_logger();

    log::info!("Application started.");

    if !Path::new(CONFIG_FILE_PATH).exists() {
        log::error!("Configuration file '{}' does not exist.", CONFIG_FILE_PATH);
        std::process::exit(1);
    }

    log::info!("Configuration file '{}' found. Proceeding with initialization.", CONFIG_FILE_PATH);

    // Lade die Konfiguration
    let config = load_config("config.json");

    // Sende Benachrichtigung, dass die Anwendung gestartet wurde
    send_start_notification();

    // Pfade definieren
    let source_path = Path::new(&config.source_folder);
    let destination_path = Path::new(&config.destination_folder);

    // Überprüfe, ob die Ordner existieren
    if !source_path.exists() {
        log::error!("Source folder does not exist: {}", config.source_folder);
        return;
    }
    if !destination_path.exists() {
        if let Err(e) = std::fs::create_dir_all(destination_path) {
            log::error!("Failed to create destination folder: {}", e);
            return;
        }
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

    // Zeige eine Benachrichtigung beim Beenden
    send_custom_notification("App beendet", "Die Anwendung wurde erfolgreich beendet.", "test");
}
