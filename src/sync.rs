use std::fs::{ create_dir_all, metadata };
use std::path::Path;
use std::time::SystemTime;
use walkdir::WalkDir;
use fs_extra::file::{ copy, CopyOptions };

pub fn sync_folders(source: &Path, destination: &Path) -> u32 {
    let mut copied_files = 0;

    for entry in WalkDir::new(source) {
        let entry = entry.unwrap();
        let relative_path = entry.path().strip_prefix(source).unwrap();
        let dest_path = destination.join(relative_path);

        if entry.path().is_dir() {
            if !dest_path.exists() {
                create_dir_all(&dest_path).expect("Failed to create directory");
            }
        } else if entry.path().is_file() {
            let should_copy = if dest_path.exists() {
                let source_metadata = metadata(entry.path()).expect("Failed to get metadata");
                let dest_metadata = metadata(&dest_path).expect("Failed to get metadata");

                let source_modified = source_metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
                let dest_modified = dest_metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);

                source_modified > dest_modified
            } else {
                true
            };

            if should_copy {
                let mut options = CopyOptions::new();
                options.overwrite = true;
                copy(entry.path(), &dest_path, &options).expect("Failed to copy file");
                copied_files += 1;
            }
        }
    }

    copied_files
}
