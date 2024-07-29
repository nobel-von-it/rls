use std::fs::Permissions;

use colored::Colorize;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Tokens {
    Dir(String),
    File(String),
}
impl From<std::fs::DirEntry> for Tokens {
    fn from(de: std::fs::DirEntry) -> Self {
        let ft = de.file_type().map_err(|e| eprintln!("error {e}")).unwrap();
        if ft.is_dir() {
            if let Some(dname) = de.file_name().to_str() {
                return Self::Dir(dname.to_string());
            } else {
                eprintln!("cant read dir name");
                std::process::exit(1);
            }
        } else {
            if let Some(fname) = de.file_name().to_str() {
                return Self::File(fname.to_string());
            } else {
                eprintln!("cant read file name");
                std::process::exit(1);
            }
        }
    }
}
impl std::fmt::Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tokens::Dir(dir) => write!(f, "{}", dir.blue().bold().to_string()),
            Tokens::File(file) => write!(f, "{}", file),
        }
    }
}
impl Tokens {
    pub fn file_name(&self) -> String {
        match self {
            Tokens::Dir(dir) => format!("{}", dir),
            Tokens::File(file) => format!("{}", file),
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Dir(d) => d.is_empty(),
            Self::File(f) => f.is_empty(),
        }
    }
    pub fn is_dot(&self) -> bool {
        match self {
            Self::Dir(d) => d.starts_with("."),
            Self::File(f) => f.starts_with("."),
        }
    }
    pub fn is_dir(&self) -> bool {
        match self {
            Self::Dir(_) => true,
            Self::File(_) => false,
        }
    }
}
