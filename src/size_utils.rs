use std::fmt;

const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];

/// A wrapper type for file sizes that provides human-readable formatting
#[derive(Debug, Clone, Copy)]
pub struct HumanSize(pub u64);

impl HumanSize {
    /// Convert bytes to the most appropriate unit
    pub fn new(size: u64) -> Self {
        HumanSize(size)
    }

    /// Get the size in a human-readable format
    pub fn format(&self) -> String {
        let mut size = self.0 as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", size as u64, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }
}

impl fmt::Display for HumanSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_size_format() {
        assert_eq!(HumanSize(0).format(), "0 B");
        assert_eq!(HumanSize(1023).format(), "1023 B");
        assert_eq!(HumanSize(1024).format(), "1.0 KB");
        assert_eq!(HumanSize(1024 * 1024).format(), "1.0 MB");
        assert_eq!(HumanSize(1024 * 1024 * 1024).format(), "1.0 GB");
        assert_eq!(HumanSize(1024 * 1024 * 1024 * 1024).format(), "1.0 TB");
    }
}
