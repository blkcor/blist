use crate::colors::ColorTheme;
use crate::file_entry::FileEntry;
use crate::size_utils::HumanSize;
use serde_json;
use std::path::Path;
use tabled::Table;
use tabled::settings::{
    Color, Modify, Style, Width,
    object::{Columns, Rows},
};

/// Print the result in table format
pub fn print_table(entries: Vec<FileEntry>, human_readable: bool, _color_theme: &ColorTheme) {
    let entries: Vec<TableEntry> = entries
        .into_iter()
        .map(|e| TableEntry::new(e, human_readable))
        .collect();

    let mut table = Table::new(entries);

    // 设置表格样式
    table
        .with(Style::modern_rounded())
        .with(Width::wrap(100))
        // 设置表头颜色为亮青色
        .with(Modify::new(Rows::first()).with(Color::FG_BRIGHT_CYAN))
        // 设置名称列为亮蓝色
        .with(Modify::new(Columns::first()).with(Color::FG_BRIGHT_BLUE))
        // 设置类型列为亮黄色
        .with(Modify::new(Columns::new(1..2)).with(Color::FG_YELLOW))
        // 设置大小列为亮洋红色
        .with(Modify::new(Columns::new(2..3)).with(Color::FG_MAGENTA))
        // 设置修改时间列为亮绿色
        .with(Modify::new(Columns::new(3..4)).with(Color::FG_GREEN))
        // 设置权限列为亮白色
        .with(Modify::new(Columns::new(4..5)).with(Color::FG_WHITE));

    println!("{}", table);
}

/// Print the result in long format (similar to ls -l)
pub fn print_long(entries: Vec<FileEntry>, human_readable: bool, color_theme: &ColorTheme) {
    println!(
        " {:10} {:8} {:8} {:>8} {:19} {}",
        "Permissions", "Owner", "Group", "Size", "Modified", "Name"
    );
    println!(" {}", "-".repeat(60));

    for entry in entries {
        let size = if human_readable {
            format!("{}", HumanSize(entry.size()))
        } else {
            entry.size().to_string()
        };

        print!(
            " {:10} {:8} {:8} {:>8} {:19} ",
            entry.permissions(),
            entry.owner().unwrap_or_default(),
            entry.group().unwrap_or_default(),
            size,
            entry.modified().format("%Y-%m-%d %H:%M:%S")
        );

        color_theme
            .print_colored(entry.name(), entry.path())
            .unwrap();
        println!();
    }
}

/// Print the result in tree format
pub fn print_tree(entries: Vec<FileEntry>, human_readable: bool, color_theme: &ColorTheme) {
    fn print_tree_recursive(
        entries: &[FileEntry],
        path: &Path,
        prefix: &str,
        is_last: bool,
        human_readable: bool,
        color_theme: &ColorTheme,
    ) {
        let entry = entries.iter().find(|e| e.path() == path).unwrap();

        // Print current entry
        print!("{}", prefix);
        print!("{}", if is_last { "└── " } else { "├── " });

        color_theme
            .print_colored(entry.name(), entry.path())
            .unwrap();

        if human_readable {
            print!(" ({})", HumanSize(entry.size()));
        }
        println!();

        // Get children
        let children: Vec<_> = entries
            .iter()
            .filter(|e| e.path().parent() == Some(path))
            .collect();

        let child_count = children.len();
        for (i, child) in children.iter().enumerate() {
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            print_tree_recursive(
                entries,
                child.path(),
                &new_prefix,
                i == child_count - 1,
                human_readable,
                color_theme,
            );
        }
    }

    // Find root entries (those without parents in the list)
    let root_entries: Vec<_> = entries
        .iter()
        .filter(|e| !entries.iter().any(|p| e.path().parent() == Some(p.path())))
        .collect();

    for (i, entry) in root_entries.iter().enumerate() {
        print_tree_recursive(
            &entries,
            entry.path(),
            "",
            i == root_entries.len() - 1,
            human_readable,
            color_theme,
        );
    }
}

/// Print the result in JSON format
pub fn print_json(entries: Vec<FileEntry>) {
    println!(
        "{}",
        serde_json::to_string_pretty(&entries)
            .unwrap_or_else(|_| "Error serializing to JSON".to_string())
    );
}

#[derive(tabled::Tabled)]
struct TableEntry {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    type_: String,
    #[tabled(rename = "Size")]
    size: String,
    #[tabled(rename = "Modified")]
    modified: String,
    #[tabled(rename = "Permissions")]
    permissions: String,
}

impl TableEntry {
    fn new(entry: FileEntry, human_readable: bool) -> Self {
        Self {
            name: entry.name().to_string(),
            type_: if entry.is_dir() {
                "Dir".into()
            } else {
                "File".into()
            },
            size: if human_readable {
                HumanSize(entry.size()).to_string()
            } else {
                entry.size().to_string()
            },
            modified: entry.modified().format("%Y-%m-%d %H:%M:%S").to_string(),
            permissions: entry.permissions(),
        }
    }
}
