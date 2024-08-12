use std::cmp::PartialEq;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::time::UNIX_EPOCH;

use chrono::DateTime;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FileKind {
    Dir,
    File,
    Symlink,
    Other,
}
impl From<std::fs::FileType> for FileKind {
    fn from(ft: std::fs::FileType) -> Self {
        if ft.is_dir() {
            Self::Dir
        } else if ft.is_file() {
            Self::File
        } else if ft.is_symlink() {
            Self::Symlink
        } else {
            Self::Other
        }
    }
}
pub enum Extensions {
    Rust,
    Toml,
    C,
    Cpp,
    CMake,
    CppHeader,
    CHeader,
    Go,
    Txt,
    Json,
    Other(String),
}
impl From<String> for Extensions {
    fn from(s: String) -> Self {
        let s = s.split_once(".").unwrap_or(("", "")).1.to_lowercase();
        if s.is_empty() {
            return Self::Other("".to_string());
        }
        match s.as_str() {
            "rs" => Self::Rust,
            "toml" => Self::Toml,
            "c" => Self::C,
            "cpp" => Self::Cpp,
            "cmake" => Self::CMake,
            "h" => Self::CHeader,
            "hpp" => Self::CppHeader,
            "go" => Self::Go,
            "txt" => Self::Txt,
            "json" => Self::Json,
            _ => Self::Other(s),
        }
    }
}
pub struct Permissions(String);
impl From<std::fs::Permissions> for Permissions {
    fn from(p: std::fs::Permissions) -> Self {
        let mode = p.mode();
        Self(format!(
            "{}{}{}{}{}{}{}{}{}",
            if mode & 0o400 != 0 { "r" } else { "-" },
            if mode & 0o200 != 0 { "w" } else { "-" },
            if mode & 0o100 != 0 { "x" } else { "-" },
            if mode & 0o040 != 0 { "r" } else { "-" },
            if mode & 0o020 != 0 { "w" } else { "-" },
            if mode & 0o010 != 0 { "x" } else { "-" },
            if mode & 0o004 != 0 { "r" } else { "-" },
            if mode & 0o002 != 0 { "w" } else { "-" },
            if mode & 0o001 != 0 { "x" } else { "-" }
        ))
    }
}
impl std::fmt::Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Permissions {
    fn is_executable(&self) -> bool {
        self.0.contains("x")
    }
}
pub struct FileInfo {
    path: String,
    name: String,
    colored_name: String,
    kind: FileKind,
    extension: Extensions,
    permissions: Permissions,
    size: u64,
    link_target: Option<String>,
    nlink: u64,
    owner: String,
    group: String,
    mtime: String,
}
impl PartialEq for FileInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for FileInfo {}

impl PartialOrd for FileInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FileInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl From<std::fs::DirEntry> for FileInfo {
    fn from(de: std::fs::DirEntry) -> Self {
        let metadata = de.metadata().unwrap();

        let name = de.file_name().to_str().unwrap().to_string();
        let path = std::fs::canonicalize(&name)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let extension = Extensions::from(name.clone());
        let kind = FileKind::from(metadata.file_type());
        let permissions = Permissions::from(metadata.permissions());
        let size = metadata.len();
        let nlink = metadata.nlink();
        let owner = metadata.uid().to_string();
        let group = metadata.gid().to_string();

        let datetime: DateTime<chrono::Local> =
            (UNIX_EPOCH + std::time::Duration::from_secs(metadata.mtime() as u64)).into();
        let mtime_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        let colored_name = match kind {
            FileKind::Dir => format!("\x1b[1;34m{}\x1b[0m", &name),
            FileKind::Symlink => format!("\x1b[1;35m{}\x1b[0m", &name),
            _ => {
                if permissions.is_executable() {
                    format!("\x1b[1;32m{}\x1b[0m", &name)
                } else {
                    name.clone()
                }
            }
        };
        let link_target = if kind == FileKind::Symlink {
            Some(format!(
                "\x1b[1;35m{}\x1b[0m",
                std::fs::read_link(&path).unwrap().to_str().unwrap(),
            ))
        } else {
            None
        };

        Self {
            path,
            name,
            colored_name,
            kind,
            extension,
            permissions,
            size,
            nlink,
            link_target,
            owner,
            group,
            mtime: mtime_str,
        }
    }
}
impl std::fmt::Display for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(link_target) = &self.link_target {
            writeln!(
                f,
                "{:<10} {:<2} {:<4} {:<4} {:<6} {:<19} {} -> {}",
                self.permissions,
                self.nlink,
                self.owner,
                self.group,
                self.size,
                self.mtime,
                self.colored_name,
                link_target
            )
        } else {
            writeln!(
                f,
                "{:<10} {:<2} {:<4} {:<4} {:<6} {:<19} {}",
                self.permissions,
                self.nlink,
                self.owner,
                self.group,
                self.size,
                self.mtime,
                self.colored_name
            )
        }
    }
}
impl FileInfo {
    pub fn colored_file_name(&self) -> String {
        self.colored_name.clone()
    }
    pub fn file_name(&self) -> String {
        self.name.clone()
    }
    pub fn is_dot(&self) -> bool {
        self.name.starts_with(".")
    }
}
pub struct Entries(Vec<FileInfo>);
impl Entries {
    pub fn new(path: &str) -> Self {
        let entries = std::fs::read_dir(path).unwrap();
        let mut file_infos = entries
            .map(|f| f.unwrap().into())
            .collect::<Vec<FileInfo>>();
        file_infos.sort();
        Self(file_infos)
    }
    pub fn show(&self) -> String {
        self.0
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
    pub fn only_names(&self) -> String {
        self.0
            .iter()
            .map(|f| f.colored_file_name())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
pub struct Output<I>
where
    I: Iterator<Item = String>,
{
    iter: I,
}
impl Output {
    pub fn new_fn<F>(e: &Entries, f: F) -> Self
    where
        F: FnMut(&FileInfo) -> String,
    {
        Self {
            iter: e.0.iter().map(f),
        }
    }
    pub fn new_color(e: &Entries) -> Self {
        Self::new_fn(e, |f| f.colored_name)
    }
    pub fn new_hide_dots(entries: &Entries) -> Self {
        Self {
            iter: entries
                .0
                .iter()
                .filter(|f| !f.name.starts_with("."))
                .map(|f| f.colored_file_name()),
        }
    }
    pub fn new_no_color(entries: &Entries) -> Self {
        Self {

        }
        Self(entries.0.iter().map(|f| f.file_name()))
    }
    pub fn show_single_row(&self) -> String {
        self.0.join(" ")
    }
    pub fn show_multiple_rows(&self) -> String {
        self.0.join("\n")
    }
}
