use chrono::{DateTime, Local};
use serde::Serialize;
use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
pub struct FileEntry {
    name: String,
    path: PathBuf,
    is_dir: bool,
    size: u64,
    #[serde(serialize_with = "serialize_datetime")]
    modified: DateTime<Local>,
    #[serde(serialize_with = "serialize_datetime_option")]
    created: Option<DateTime<Local>>,
    permissions: u32,
    owner: Option<String>,
    group: Option<String>,
}

fn serialize_datetime<S>(dt: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&dt.format("%Y-%m-%d %H:%M:%S").to_string())
}

fn serialize_datetime_option<S>(
    dt: &Option<DateTime<Local>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match dt {
        Some(dt) => serialize_datetime(dt, serializer),
        None => serializer.serialize_none(),
    }
}

impl FileEntry {
    pub fn new(
        path: PathBuf,
        is_dir: bool,
        size: u64,
        modified: Option<DateTime<Local>>,
        created: Option<DateTime<Local>>,
    ) -> Self {
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();

        Self {
            name,
            path,
            is_dir,
            size,
            modified: modified.unwrap_or_else(|| Local::now()),
            created,
            permissions: 0,
            owner: None,
            group: None,
        }
    }

    pub fn from_metadata(path: PathBuf, metadata: &Metadata) -> Self {
        let is_dir = metadata.is_dir();
        let size = metadata.len();
        let modified = metadata
            .modified()
            .ok()
            .map(|t| DateTime::from(t))
            .unwrap_or_else(|| Local::now());
        let created = metadata.created().ok().map(|t| DateTime::from(t));
        let permissions = metadata.mode();

        let mut entry = Self::new(path, is_dir, size, Some(modified), created);
        entry.permissions = permissions;

        // Try to get owner and group names
        #[cfg(unix)]
        {
            use users::{get_group_by_gid, get_user_by_uid};
            if let Some(user) = get_user_by_uid(metadata.uid()) {
                entry.owner = Some(user.name().to_string_lossy().into_owned());
            }
            if let Some(group) = get_group_by_gid(metadata.gid()) {
                entry.group = Some(group.name().to_string_lossy().into_owned());
            }
        }

        entry
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn modified(&self) -> DateTime<Local> {
        self.modified
    }

    pub fn created(&self) -> Option<DateTime<Local>> {
        self.created
    }

    pub fn extension(&self) -> Option<&str> {
        self.path.extension().and_then(|s| s.to_str())
    }

    pub fn permissions(&self) -> String {
        let mut perms = String::with_capacity(10);
        perms.push(if self.is_dir { 'd' } else { '-' });

        // Owner permissions
        perms.push(if self.permissions & 0o400 != 0 {
            'r'
        } else {
            '-'
        });
        perms.push(if self.permissions & 0o200 != 0 {
            'w'
        } else {
            '-'
        });
        perms.push(if self.permissions & 0o100 != 0 {
            'x'
        } else {
            '-'
        });

        // Group permissions
        perms.push(if self.permissions & 0o040 != 0 {
            'r'
        } else {
            '-'
        });
        perms.push(if self.permissions & 0o020 != 0 {
            'w'
        } else {
            '-'
        });
        perms.push(if self.permissions & 0o010 != 0 {
            'x'
        } else {
            '-'
        });

        // Others permissions
        perms.push(if self.permissions & 0o004 != 0 {
            'r'
        } else {
            '-'
        });
        perms.push(if self.permissions & 0o002 != 0 {
            'w'
        } else {
            '-'
        });
        perms.push(if self.permissions & 0o001 != 0 {
            'x'
        } else {
            '-'
        });

        perms
    }

    pub fn owner(&self) -> Option<&str> {
        self.owner.as_deref()
    }

    pub fn group(&self) -> Option<&str> {
        self.group.as_deref()
    }
}
