use crate::file_entry::FileEntry;
use owo_colors::OwoColorize;
use serde_json;
use tabled::settings::{Color, Style};
use tabled::settings::object::{Columns, Rows};
use tabled::Table;

/// Print the result in table format
pub fn print_table(file_entries: Vec<FileEntry>) {
    let mut tabled_files = Table::new(file_entries);
    tabled_files.with(Style::modern_rounded());
    tabled_files.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
    tabled_files.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
    tabled_files.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW);
    tabled_files.modify(Rows::first(), Color::FG_BRIGHT_GREEN);
    println!("{}", tabled_files);
}

/// Print the result in JSON format
pub fn print_json(file_entries: Vec<FileEntry>) {
    println!(
        "{}",
        serde_json::to_string_pretty(&file_entries)
            .unwrap_or("Error serializing to JSON".red().to_string())
    );
} 
