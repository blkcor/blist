use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = "The better ls command line.")]
pub struct CLI {
    /// The path to the directory to list files from
    #[arg(short, long, value_name = "PATH", value_hint = clap::ValueHint::DirPath)]
    pub path: Option<PathBuf>,

    /// Output format (table, long, tree, json)
    #[arg(short, long, value_name = "FORMAT", default_value = "table")]
    pub format: String,

    /// Show hidden files and directories
    #[arg(short, long)]
    pub all: bool,

    /// Recursively list subdirectories
    #[arg(short, long)]
    pub recursive: bool,

    /// Maximum recursion depth
    #[arg(short = 'd', long, value_name = "DEPTH")]
    pub max_depth: Option<usize>,

    /// Sort field (name, size, modified, created, type)
    #[arg(short, long, value_name = "FIELD", default_value = "name")]
    pub sort: String,

    /// Sort order (asc, desc)
    #[arg(short, long, value_name = "ORDER", default_value = "asc")]
    pub order: String,

    /// Filter by file extensions (comma-separated)
    #[arg(short, long, value_name = "EXTS", value_delimiter = ',')]
    pub extensions: Option<Vec<String>>,

    /// Minimum file size in bytes
    #[arg(long, value_name = "SIZE")]
    pub min_size: Option<u64>,

    /// Maximum file size in bytes
    #[arg(long, value_name = "SIZE")]
    pub max_size: Option<u64>,

    /// Use human-readable file sizes
    #[arg(short = 'H', long)]
    pub human_readable: bool,

    /// Disable color output
    #[arg(long)]
    pub no_color: bool,

    /// Show only directories
    #[arg(long)]
    pub dirs_only: bool,

    /// Show only files
    #[arg(long)]
    pub files_only: bool,

    /// Filter files using glob pattern
    #[arg(short, long, value_name = "PATTERN")]
    pub glob: Option<String>,

    /// Show summary statistics
    #[arg(long)]
    pub summary: bool,
}
