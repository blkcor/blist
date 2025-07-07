use crate::file_entry::{FileEntry, EntryType};
use chrono::{DateTime, Utc};
use owo_colors::OwoColorize;
use std::fs;
use std::fs::Metadata;
use std::path::Path;

/// Get the files in the given path.
/// # Arguments
/// * `path` - The path to the directory to list files from.
/// # Returns
/// A vector of FileEntry containing the information about files in the directory.
pub fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry_result in read_dir {
            if let Ok(entry) = entry_result {
                if let Ok(metadata) = fs::metadata(&entry.path()) {
                    data.push(map_data(&entry, metadata))
                } else {
                    println!(
                        "{:?}",
                        format!("Error reading metadata for file: {}", get_file_name(&entry)).red()
                    );
                }
            }
        }
    }
    data
}

/// Map the file entry data to the FileEntry struct
fn map_data(entry: &fs::DirEntry, metadata: Metadata) -> FileEntry {
    FileEntry {
        name: get_file_name(&entry),
        entry_type: if metadata.is_dir(){
            EntryType::Dir
        } else {
            EntryType::File
        },
        len_bytes: metadata.len(),
        modified: metadata.modified().map_or_else(
            |_| "".to_string(),
            |time| {
                let date: DateTime<Utc> = time.into();
                format!("{}", date.format("%Y-%m-%d %H:%M:%S"))
            },
        ),
    }
}

/// Extract the file name from the DirEntry
fn get_file_name(file_entry: &fs::DirEntry) -> String {
    file_entry
        .file_name()
        .into_string()
        .unwrap_or("unknown name".into())
} 
