use crate::file_entry::FileEntry;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortField {
    Name,
    Size,
    Modified,
    Created,
    Type,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl SortField {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "name" => Some(SortField::Name),
            "size" => Some(SortField::Size),
            "modified" => Some(SortField::Modified),
            "created" => Some(SortField::Created),
            "type" => Some(SortField::Type),
            _ => None,
        }
    }
}

impl SortOrder {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "asc" | "ascending" => Some(SortOrder::Ascending),
            "desc" | "descending" => Some(SortOrder::Descending),
            _ => None,
        }
    }
}

pub fn sort_entries(entries: &mut Vec<FileEntry>, field: SortField, order: SortOrder) {
    entries.sort_by(|a, b| {
        let cmp = match field {
            SortField::Name => natural_sort(a.name(), b.name()),
            SortField::Size => a.size().cmp(&b.size()),
            SortField::Modified => a.modified().cmp(&b.modified()),
            SortField::Created => a.created().cmp(&b.created()),
            SortField::Type => {
                // Sort directories first, then by extension
                match (a.is_dir(), b.is_dir()) {
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    _ => {
                        let a_ext = a.extension().unwrap_or_default();
                        let b_ext = b.extension().unwrap_or_default();
                        a_ext.cmp(b_ext)
                    }
                }
            }
        };

        match order {
            SortOrder::Ascending => cmp,
            SortOrder::Descending => cmp.reverse(),
        }
    });
}

/// Natural sort comparison for strings containing numbers
fn natural_sort(a: &str, b: &str) -> Ordering {
    let mut a_chars = a.chars().peekable();
    let mut b_chars = b.chars().peekable();

    loop {
        match (a_chars.peek(), b_chars.peek()) {
            (None, None) => return Ordering::Equal,
            (None, _) => return Ordering::Less,
            (_, None) => return Ordering::Greater,
            (Some(ac), Some(bc)) => {
                if ac.is_digit(10) && bc.is_digit(10) {
                    // Compare numbers
                    let a_num = extract_number(&mut a_chars);
                    let b_num = extract_number(&mut b_chars);
                    match a_num.cmp(&b_num) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                } else {
                    // Compare characters
                    match ac.cmp(bc) {
                        Ordering::Equal => {
                            a_chars.next();
                            b_chars.next();
                            continue;
                        }
                        other => return other,
                    }
                }
            }
        }
    }
}

fn extract_number<I>(chars: &mut std::iter::Peekable<I>) -> u64
where
    I: Iterator<Item = char>,
{
    let mut number = 0u64;
    while let Some(c) = chars.peek() {
        if c.is_digit(10) {
            if let Some(digit) = c.to_digit(10) {
                number = number * 10 + digit as u64;
                chars.next();
            }
        } else {
            break;
        }
    }
    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_natural_sort() {
        let mut files = vec!["file10.txt", "file2.txt", "file1.txt", "file20.txt"];
        files.sort_by(|a, b| natural_sort(a, b));
        assert_eq!(
            files,
            vec!["file1.txt", "file2.txt", "file10.txt", "file20.txt",]
        );
    }

    #[test]
    fn test_sort_field_from_str() {
        assert_eq!(SortField::from_str("name"), Some(SortField::Name));
        assert_eq!(SortField::from_str("size"), Some(SortField::Size));
        assert_eq!(SortField::from_str("modified"), Some(SortField::Modified));
        assert_eq!(SortField::from_str("invalid"), None);
    }

    #[test]
    fn test_sort_order_from_str() {
        assert_eq!(SortOrder::from_str("asc"), Some(SortOrder::Ascending));
        assert_eq!(SortOrder::from_str("desc"), Some(SortOrder::Descending));
        assert_eq!(SortOrder::from_str("invalid"), None);
    }
}
