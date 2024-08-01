use std::cmp::PartialEq;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::time::UNIX_EPOCH;

use chrono::DateTime;

// #[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
// pub enum Tokens {
//     Dir(String),
//     File(String),
// }
// impl From<std::fs::DirEntry> for Tokens {
//     fn from(de: std::fs::DirEntry) -> Self {
//         let ft = de.file_type().map_err(|e| eprintln!("error {e}")).unwrap();
//         if ft.is_dir() {
//             if let Some(dname) = de.file_name().to_str() {
//                 return Self::Dir(dname.to_string());
//             } else {
//                 eprintln!("cant read dir name");
//                 std::process::exit(1);
//             }
//         } else {
//             if let Some(fname) = de.file_name().to_str() {
//                 return Self::File(fname.to_string());
//             } else {
//                 eprintln!("cant read file name");
//                 std::process::exit(1);
//             }
//         }
//     }
// }
// impl std::fmt::Display for Tokens {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Tokens::Dir(dir) => write!(f, "{}", dir),
//             Tokens::File(file) => write!(f, "{}", file),
//         }
//     }
// }
// impl Tokens {
//     pub fn file_name(&self) -> String {
//         match self {
//             Tokens::Dir(dir) => format!("{}", dir),
//             Tokens::File(file) => format!("{}", file),
//         }
//     }
//     pub fn add_color(&self) -> String {
//         match self {
//             Tokens::Dir(dir) => format!("{}", dir.blue().bold().to_string()),
//             Tokens::File(file) => format!("{}", file),
//         }
//     }
//     pub fn is_empty(&self) -> bool {
//         match self {
//             Self::Dir(d) => d.is_empty(),
//             Self::File(f) => f.is_empty(),
//         }
//     }
//     pub fn is_dot(&self) -> bool {
//         match self {
//             Self::Dir(d) => d.starts_with("."),
//             Self::File(f) => f.starts_with("."),
//         }
//     }
//     pub fn is_dir(&self) -> bool {
//         match self {
//             Self::Dir(_) => true,
//             Self::File(_) => false,
//         }
//     }
// }
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
    pub path: String,
    pub name: String,
    pub colored_name: String,
    pub kind: FileKind,
    pub extension: Extensions,
    pub permissions: Permissions,
    pub size: u64,
    pub link_target: Option<String>,
    pub nlink: u64,
    pub owner: String,
    pub group: String,
    pub mtime: String,
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
pub struct Output(Vec<String>);
impl Output {
    pub fn new_full_info(entries: &Entries) -> Self {
        Self(
            entries
                .0
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>(),
        )
    }
    pub fn new_path(entries: &Entries) -> Self {
        Self(
            entries
                .0
                .iter()
                .map(|f| f.path.clone())
                .collect::<Vec<String>>(),
        )
    }
    pub fn new_color(entries: &Entries) -> Self {
        Self(
            entries
                .0
                .iter()
                .map(|f| f.colored_file_name())
                .collect::<Vec<String>>(),
        )
    }
    pub fn new_hide_dots(entries: &Entries) -> Self {
        Self(
            entries
                .0
                .iter()
                .filter(|f| !f.name.starts_with("."))
                .map(|f| f.colored_file_name())
                .collect::<Vec<String>>(),
        )
    }
    pub fn new_no_color(entries: &Entries) -> Self {
        Self(
            entries
                .0
                .iter()
                .map(|f| f.file_name())
                .collect::<Vec<String>>(),
        )
    }
    pub fn show_single_row(&self) -> String {
        self.0.join(" ")
    }
    pub fn show_multiple_rows(&self) -> String {
        self.0.join("\n")
    }
}
