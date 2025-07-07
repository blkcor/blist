use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = "The better ls command line.")]
pub struct CLI {
    /// The path to the directory to list files from.
    #[arg(short, long, value_name = "PATH", value_hint = clap::ValueHint::DirPath)]
    pub path: Option<PathBuf>,

    /// Output in JSON format
    #[arg(short, long)]
    pub json: bool,
} 
