use serde::Deserialize;
use std::fs;
use std::fs::{create_dir_all, metadata};
use std::path::Path;
use walkdir::WalkDir;
use fs_extra::file::{copy, CopyOptions};

// Struktur für Konfigurationsdaten
#[derive(Deserialize)]
struct Config {
    source_folder: String,
    destination_folder: String,
}

// Funktion, um die Konfiguration aus einer Datei zu laden
fn load_config(file_path: &str) -> Config {
    let config_data = fs::read_to_string(file_path).expect("Unable to read config file");
    serde_json::from_str(&config_data).expect("JSON was not well-formatted")
}

// Funktion zur Synchronisierung von Ordnern
fn sync_folders(source: &Path, destination: &Path) -> u32 {
    let mut copied_files = 0; // Zähler für kopierte Dateien

    for entry in WalkDir::new(source) {
        let entry = entry.unwrap();
        let relative_path = entry.path().strip_prefix(source).unwrap();
        let dest_path = destination.join(relative_path);

        if entry.path().is_dir() {
            // Erstelle Ordner im Ziel, falls sie nicht existieren
            if !dest_path.exists() {
                create_dir_all(&dest_path).expect("Failed to create directory");
            }
        } else if entry.path().is_file() {
            let should_copy = if dest_path.exists() {
                // Vergleiche die Änderungszeit
                let source_metadata = metadata(entry.path()).expect(
                    "Failed to get metadata of source file",
                );
                let dest_metadata = metadata(&dest_path).expect(
                    "Failed to get metadata of destination file",
                );

                let source_modified = source_metadata
                    .modified()
                    .expect("Failed to get source modification time");
                let dest_modified = dest_metadata
                    .modified()
                    .expect("Failed to get destination modification time");

                source_modified > dest_modified // Kopiere nur, wenn die Quelldatei neuer ist
            } else {
                true // Kopiere, wenn die Datei im Ziel nicht existiert
            };

            if should_copy {
                let mut options = CopyOptions::new(); // Standardoptionen
                options.overwrite = true;
                copy(entry.path(), &dest_path, &options).expect("Failed to copy file");
                copied_files += 1; // Zähler inkrementieren
            }
        }
    }

    copied_files // Gibt die Anzahl der kopierten Dateien zurück
}

fn main() {
    // Lade die Konfiguration
    let config = load_config("config.json");

    // Pfade definieren
    let source_path = Path::new(&config.source_folder);
    let destination_path = Path::new(&config.destination_folder);

    // Überprüfe, ob die Ordner existieren
    if !source_path.exists() {
        eprintln!("Source folder does not exist: {}", config.source_folder);
        return;
    }
    if !destination_path.exists() {
        create_dir_all(destination_path).expect("Failed to create destination folder");
    }

    // Synchronisiere die Ordner und erhalte die Anzahl kopierter Dateien
    let copied_files = sync_folders(source_path, destination_path);

    // Ausgabe der Ergebnisse
    println!(
        "Synchronization complete. {} files were copied.",
        copied_files
    );
}
