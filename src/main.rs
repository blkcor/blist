use chrono::{DateTime, Utc};
use clap::Parser;
use owo_colors::OwoColorize;
use serde::Serialize;
use std::fs;
use std::fs::Metadata;
use std::path::{Path, PathBuf};
use strum_macros::Display;
use tabled::settings::{Color, Style};
use tabled::settings::object::{Columns, Rows};
use tabled::{Table, Tabled};

#[derive(Debug, Display, Serialize)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Tabled, Serialize)]
struct FileEntry {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    entry_type: EntryType,
    #[tabled(rename = "Length(Byte)")]
    len_bytes: u64,
    #[tabled(rename = "Modified")]
    modified: String,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = "The better ls command line.")]
struct CLI {
    /// The path to the directory to list files from.
    #[arg(short, long, value_name = "PATH", value_hint = clap::ValueHint::DirPath)]
    path: Option<PathBuf>,

    /// Output in JSON format
    #[arg(short, long)]
    json: bool,
}
fn main() {
    let cli = CLI::parse();
    let path = cli.path.unwrap_or(PathBuf::from("."));
    // check if the path exists
    if let Ok(path_exists) = fs::exists(&path) {
        if path_exists {
            // with the correct path
            let file_entries = get_files(&path);
            if cli.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&file_entries)
                        .unwrap_or("Error serializing to JSON".red().to_string())
                )
            } else {
                // print table format default
                print_table(file_entries);
            }
        } else {
            println!("{:?}", "Path does not exists".red());
        }
    } else {
        println!("{:?}", "Error checking path".red());
    }
}

/// Get the files in the given path.
/// # Arguments
/// * `path` - The path to the directory to list files from.
/// # Returns
/// A vector of strings containing the names of the files in the directory.
fn get_files(path: &Path) -> Vec<FileEntry> {
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

// Map the file entry data to the FileEntry struct
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

// extract the file name from the DirEntry
fn get_file_name(file_entry: &fs::DirEntry) -> String {
    file_entry
        .file_name()
        .into_string()
        .unwrap_or("unknown name".into())
}

// print the result as table format
fn print_table(file_entries: Vec<FileEntry>) {
    let mut tabled_files = Table::new(file_entries);
    tabled_files.with(Style::modern_rounded());
    tabled_files.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
    tabled_files.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
    tabled_files.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW);
    tabled_files.modify(Rows::first(), Color::FG_BRIGHT_GREEN);
    println!("{}", tabled_files);
}
