use serde::Serialize;
use strum_macros::Display;
use tabled::Tabled;

#[derive(Debug, Display, Serialize)]
pub enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Tabled, Serialize)]
pub struct FileEntry {
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "Type")]
    pub entry_type: EntryType,
    #[tabled(rename = "Length(Byte)")]
    pub len_bytes: u64,
    #[tabled(rename = "Modified")]
    pub modified: String,
} 
