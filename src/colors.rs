use std::io::Write;
use std::path::Path;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct ColorTheme {
    enabled: bool,
}

impl ColorTheme {
    pub fn new(color_enabled: bool) -> Self {
        Self {
            enabled: color_enabled,
        }
    }

    /// Returns whether colors are enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn print_colored<P: AsRef<Path>>(&self, text: &str, path: P) -> std::io::Result<()> {
        let mut stdout = StandardStream::stdout(if self.enabled {
            ColorChoice::Auto
        } else {
            ColorChoice::Never
        });

        let path_ref = path.as_ref();
        let color = self.get_color_for_path(path_ref);
        let mut color_spec = ColorSpec::new();

        if let Some(c) = color {
            color_spec.set_fg(Some(c));
            if self.should_be_bold(path_ref) {
                color_spec.set_bold(true);
            }
        }

        stdout.set_color(&color_spec)?;
        write!(&mut stdout, "{}", text)?;
        stdout.reset()?;
        Ok(())
    }

    fn get_color_for_path<P: AsRef<Path>>(&self, path: P) -> Option<Color> {
        if !self.enabled {
            return None;
        }

        let path = path.as_ref();

        if path.is_dir() {
            return Some(Color::Blue);
        }

        // Check if file is executable
        #[cfg(unix)]
        if let Ok(metadata) = path.metadata() {
            use std::os::unix::fs::PermissionsExt;
            if metadata.permissions().mode() & 0o111 != 0 {
                return Some(Color::Green);
            }
        }

        match path.extension().and_then(|s| s.to_str()) {
            // Source code files
            Some("rs") => Some(Color::Red),
            Some("py") => Some(Color::Yellow),
            Some("js") | Some("ts") => Some(Color::Yellow),

            // Configuration files
            Some("json") | Some("yaml") | Some("yml") | Some("toml") | Some("ini") => {
                Some(Color::Yellow)
            }

            // Documentation files
            Some("md") | Some("txt") | Some("rst") | Some("doc") | Some("docx") => {
                Some(Color::White)
            }

            // Image files
            Some("jpg") | Some("jpeg") | Some("png") | Some("gif") | Some("bmp") => {
                Some(Color::Magenta)
            }

            // Compressed files
            Some("zip") | Some("tar") | Some("gz") | Some("xz") | Some("bz2" | "7z") => {
                Some(Color::Red)
            }

            // Default - no color
            _ => None,
        }
    }

    fn should_be_bold(&self, path: &Path) -> bool {
        if !self.enabled {
            return false;
        }

        if path.is_dir() {
            return true;
        }

        #[cfg(unix)]
        if let Ok(metadata) = path.metadata() {
            use std::os::unix::fs::PermissionsExt;
            if metadata.permissions().mode() & 0o111 != 0 {
                return true;
            }
        }

        path.extension()
            .and_then(|s| s.to_str())
            .map(|ext| matches!(ext, "zip" | "tar" | "gz" | "xz" | "bz2" | "7z"))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::TempDir;

    #[test]
    fn test_color_selection() {
        let temp_dir = TempDir::new().unwrap();
        let theme = ColorTheme::new(true);

        // Create and test directory
        let dir_path = temp_dir.path().join("test_dir");
        fs::create_dir(&dir_path).unwrap();
        assert_eq!(theme.get_color_for_path(&dir_path), Some(Color::Blue));

        // Create and test Rust file
        let rust_file = temp_dir.path().join("main.rs");
        File::create(&rust_file).unwrap();
        assert_eq!(theme.get_color_for_path(&rust_file), Some(Color::Red));

        // Create and test image file
        let image_file = temp_dir.path().join("photo.jpg");
        File::create(&image_file).unwrap();
        assert_eq!(theme.get_color_for_path(&image_file), Some(Color::Magenta));

        // Create and test archive file
        let archive_file = temp_dir.path().join("data.zip");
        File::create(&archive_file).unwrap();
        assert_eq!(theme.get_color_for_path(&archive_file), Some(Color::Red));

        // Test color disabled
        let theme = ColorTheme::new(false);
        assert_eq!(theme.get_color_for_path(&dir_path), None);
    }

    #[test]
    fn test_should_be_bold() {
        let temp_dir = TempDir::new().unwrap();
        let theme = ColorTheme::new(true);

        // Create and test directory
        let dir_path = temp_dir.path().join("test_dir");
        fs::create_dir(&dir_path).unwrap();
        assert!(theme.should_be_bold(&dir_path));

        // Create and test archive file
        let archive_file = temp_dir.path().join("data.zip");
        File::create(&archive_file).unwrap();
        assert!(theme.should_be_bold(&archive_file));

        // Create and test regular file
        let regular_file = temp_dir.path().join("test.txt");
        File::create(&regular_file).unwrap();
        assert!(!theme.should_be_bold(&regular_file));

        // Test color disabled
        let theme = ColorTheme::new(false);
        assert!(!theme.should_be_bold(&dir_path));
    }
}
