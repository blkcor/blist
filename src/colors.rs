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
            Some("zip") | Some("tar") | Some("gz") | Some("xz") | Some("bz2") | Some("7z") => {
                Some(Color::Red)
            }

            // Default - no color
            _ => None,
        }
    }

    fn should_be_bold(&self, path: &Path) -> bool {
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
    use std::path::PathBuf;

    #[test]
    fn test_color_selection() {
        let theme = ColorTheme::new(true);

        // Test directory
        let dir = PathBuf::from("test_dir");
        assert_eq!(theme.get_color_for_path(&dir), Some(Color::Blue));

        // Test Rust file
        let rust_file = PathBuf::from("main.rs");
        assert_eq!(theme.get_color_for_path(&rust_file), Some(Color::Red));

        // Test image file
        let image_file = PathBuf::from("photo.jpg");
        assert_eq!(theme.get_color_for_path(&image_file), Some(Color::Magenta));

        // Test archive file
        let archive_file = PathBuf::from("data.zip");
        assert_eq!(theme.get_color_for_path(&archive_file), Some(Color::Red));
    }
}
