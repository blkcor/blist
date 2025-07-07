use crate::file_entry::FileEntry;
use glob::Pattern;
use std::path::Path;

pub struct FileFilter {
    extensions: Option<Vec<String>>,
    min_size: Option<u64>,
    max_size: Option<u64>,
    dirs_only: bool,
    files_only: bool,
    glob_pattern: Option<Pattern>,
    show_hidden: bool,
}

impl FileFilter {
    pub fn new() -> Self {
        Self {
            extensions: None,
            min_size: None,
            max_size: None,
            dirs_only: false,
            files_only: false,
            glob_pattern: None,
            show_hidden: false,
        }
    }

    pub fn with_extensions(mut self, exts: Vec<String>) -> Self {
        self.extensions = Some(exts.into_iter().map(|e| e.to_lowercase()).collect());
        self
    }

    pub fn with_size_range(mut self, min: Option<u64>, max: Option<u64>) -> Self {
        self.min_size = min;
        self.max_size = max;
        self
    }

    pub fn dirs_only(mut self) -> Self {
        self.dirs_only = true;
        self.files_only = false;
        self
    }

    pub fn files_only(mut self) -> Self {
        self.files_only = true;
        self.dirs_only = false;
        self
    }

    pub fn with_glob(mut self, pattern: &str) -> Result<Self, glob::PatternError> {
        self.glob_pattern = Some(Pattern::new(pattern)?);
        Ok(self)
    }

    pub fn show_hidden(mut self, show: bool) -> Self {
        self.show_hidden = show;
        self
    }

    pub fn matches(&self, entry: &FileEntry) -> bool {
        // Hidden files
        if !self.show_hidden && is_hidden(entry.path()) {
            return false;
        }

        // Directory/file filtering
        if self.dirs_only && !entry.is_dir() {
            return false;
        }
        if self.files_only && entry.is_dir() {
            return false;
        }

        // Extension filtering
        if let Some(ref exts) = self.extensions {
            if !entry.is_dir() {
                if let Some(ext) = entry.extension() {
                    if !exts.contains(&ext.to_lowercase()) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        // Size filtering
        if let Some(min) = self.min_size {
            if entry.size() < min {
                return false;
            }
        }
        if let Some(max) = self.max_size {
            if entry.size() > max {
                return false;
            }
        }

        // Glob pattern matching
        if let Some(ref pattern) = self.glob_pattern {
            if !pattern.matches(entry.name()) {
                return false;
            }
        }

        true
    }
}

fn is_hidden<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref()
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

pub fn filter_entries(entries: Vec<FileEntry>, filter: &FileFilter) -> Vec<FileEntry> {
    entries.into_iter().filter(|e| filter.matches(e)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_extension_filter() {
        let filter = FileFilter::new().with_extensions(vec!["rs".to_string()]);

        let rust_file = FileEntry::new(PathBuf::from("test.rs"), false, 100, None, None);
        let txt_file = FileEntry::new(PathBuf::from("test.txt"), false, 100, None, None);

        assert!(filter.matches(&rust_file));
        assert!(!filter.matches(&txt_file));
    }

    #[test]
    fn test_size_filter() {
        let filter = FileFilter::new().with_size_range(Some(50), Some(150));

        let small_file = FileEntry::new(PathBuf::from("small.txt"), false, 40, None, None);
        let medium_file = FileEntry::new(PathBuf::from("medium.txt"), false, 100, None, None);
        let large_file = FileEntry::new(PathBuf::from("large.txt"), false, 200, None, None);

        assert!(!filter.matches(&small_file));
        assert!(filter.matches(&medium_file));
        assert!(!filter.matches(&large_file));
    }

    #[test]
    fn test_type_filter() {
        let dir_filter = FileFilter::new().dirs_only();
        let file_filter = FileFilter::new().files_only();

        let dir = FileEntry::new(PathBuf::from("test_dir"), true, 0, None, None);
        let file = FileEntry::new(PathBuf::from("test.txt"), false, 100, None, None);

        assert!(dir_filter.matches(&dir));
        assert!(!dir_filter.matches(&file));
        assert!(!file_filter.matches(&dir));
        assert!(file_filter.matches(&file));
    }

    #[test]
    fn test_glob_filter() {
        let filter = FileFilter::new().with_glob("test*.txt").unwrap();

        let matching_file = FileEntry::new(PathBuf::from("test1.txt"), false, 100, None, None);
        let non_matching_file = FileEntry::new(PathBuf::from("other.txt"), false, 100, None, None);

        assert!(filter.matches(&matching_file));
        assert!(!filter.matches(&non_matching_file));
    }

    #[test]
    fn test_hidden_filter() {
        let filter = FileFilter::new().show_hidden(false);

        let hidden_file = FileEntry::new(PathBuf::from(".hidden.txt"), false, 100, None, None);
        let normal_file = FileEntry::new(PathBuf::from("visible.txt"), false, 100, None, None);

        assert!(!filter.matches(&hidden_file));
        assert!(filter.matches(&normal_file));
    }
}
