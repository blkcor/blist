use bls::{CLI, get_files, print_table, print_json};
use clap::Parser;
use owo_colors::OwoColorize;
use std::fs;
use std::path::PathBuf;


fn main() {
    let cli = CLI::parse();
    let path = cli.path.unwrap_or(PathBuf::from("."));
    
    // Check if the path exists
    if let Ok(path_exists) = fs::exists(&path) {
        if path_exists {
            // With the correct path
            let file_entries = get_files(&path);
            if cli.json {
                print_json(file_entries);
            } else {
                // Print table format default
                print_table(file_entries);
            }
        } else {
            println!("{:?}", "Path does not exist".red());
        }
    } else {
        println!("{:?}", "Error checking path".red());
    }
}
