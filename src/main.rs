use clap::Parser;
use ls_plus::{
    CLI, ColorTheme, FileFilter, HumanSize, SortField, SortOrder, filter_entries, get_files,
    get_files_recursive, print_json, print_long, print_table, print_tree, sort_entries,
};
use std::path::PathBuf;
use std::process;

fn print_summary(entries: &[ls_plus::FileEntry]) {
    let total_files = entries.iter().filter(|e| !e.is_dir()).count();
    let total_dirs = entries.iter().filter(|e| e.is_dir()).count();
    let total_size: u64 = entries.iter().map(|e| e.size()).sum();
    let hidden_count = entries.iter().filter(|e| e.name().starts_with('.')).count();

    println!("\nSummary:");
    println!("  Files: {}", total_files);
    println!("  Directories: {}", total_dirs);
    println!("  Total Size: {}", HumanSize(total_size));
    println!("\nFile Statistics:");
    println!("  Files: {}", total_files);
    println!("  Directories: {}", total_dirs);
    println!("  Hidden: {}", hidden_count);
    println!("  Total Size: {}", HumanSize(total_size));
}

fn main() {
    let cli = CLI::parse();
    let path = cli.path.unwrap_or(PathBuf::from("."));

    // Check if the path exists
    if !path.exists() {
        eprintln!("Error: Path does not exist: {:?}", path);
        process::exit(1);
    }

    // Get files with recursion if specified
    let mut entries = if cli.recursive {
        get_files_recursive(&path, cli.max_depth.unwrap_or(std::usize::MAX))
    } else {
        get_files(&path)
    };

    // Apply filters
    let filter = FileFilter::new()
        .show_hidden(cli.all)
        .with_size_range(cli.min_size, cli.max_size);

    let filter = if let Some(exts) = cli.extensions {
        filter.with_extensions(exts)
    } else {
        filter
    };

    let filter = if cli.dirs_only {
        filter.dirs_only()
    } else if cli.files_only {
        filter.files_only()
    } else {
        filter
    };

    let filter = if let Some(pattern) = cli.glob {
        match filter.with_glob(&pattern) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error: Invalid glob pattern: {}", e);
                process::exit(1);
            }
        }
    } else {
        filter
    };

    entries = filter_entries(entries, &filter);

    // Apply sorting
    let sort_field = match SortField::from_str(&cli.sort) {
        Some(field) => field,
        None => {
            eprintln!("Error: Invalid sort field: {}", cli.sort);
            process::exit(1);
        }
    };

    let sort_order = match SortOrder::from_str(&cli.order) {
        Some(order) => order,
        None => {
            eprintln!("Error: Invalid sort order: {}", cli.order);
            process::exit(1);
        }
    };

    sort_entries(&mut entries, sort_field, sort_order);

    // Setup color theme
    let color_theme = ColorTheme::new(!cli.no_color);

    // Print summary if requested
    if cli.summary {
        print_summary(&entries);
    }

    // Print output
    match cli.format.to_lowercase().as_str() {
        "json" => print_json(entries),
        "table" => print_table(entries, cli.human_readable, &color_theme),
        "long" => print_long(entries, cli.human_readable, &color_theme),
        "tree" => print_tree(entries, cli.human_readable, &color_theme),
        _ => {
            eprintln!("Error: Invalid format: {}", cli.format);
            process::exit(1);
        }
    }
}
