use crate::file_entry::FileEntry;
use std::fs;
use std::path::Path;

/// Get the files in the given path.
/// # Arguments
/// * `path` - The path to the directory to list files from.
/// # Returns
/// A vector of FileEntry containing the information about files in the directory.
pub fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut entries = Vec::new();

    if let Ok(read_dir) = fs::read_dir(path) {
        for entry_result in read_dir {
            if let Ok(entry) = entry_result {
                if let Ok(metadata) = entry.metadata() {
                    entries.push(FileEntry::from_metadata(entry.path(), &metadata));
                }
            }
        }
    }

    entries
}

/// Get files recursively from a directory up to a maximum depth
pub fn get_files_recursive(path: &Path, max_depth: usize) -> Vec<FileEntry> {
    fn recurse(path: &Path, max_depth: usize, current_depth: usize) -> Vec<FileEntry> {
        if current_depth > max_depth {
            return Vec::new();
        }

        let mut entries = get_files(path);
        let mut subdirs = Vec::new();

        for entry in &entries {
            if entry.is_dir() {
                let subpath = path.join(entry.name());
                let mut subentries = recurse(&subpath, max_depth, current_depth + 1);
                subdirs.append(&mut subentries);
            }
        }

        entries.append(&mut subdirs);
        entries
    }

    recurse(path, max_depth, 0)
}
