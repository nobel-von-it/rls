// use std::os::unix::fs::PermissionsExt;
// use regex::Regex;
//
// use crate::{files::Tokens, parser::Config};
//
// pub struct Entries(Vec<Tokens>);
// impl Entries {
//     pub fn new(config: &Config) -> Option<Self> {
//         let dir = match std::fs::read_dir(&config.dir_name) {
//             Ok(rd) => rd,
//             Err(_) => return None,
//         };
//
//         Some(Self(
//             dir.map(|f| f.unwrap())
//                 .map(Tokens::from)
//                 .filter(|f| !f.is_empty())
//                 .collect::<Vec<Tokens>>(),
//         ))
//     }
//     pub fn hide_dots(&self) -> Self {
//         Self(
//             self.0
//                 .iter()
//                 .filter(|f| !f.is_dot())
//                 .map(|f| f.clone())
//                 .collect(),
//         )
//     }
//     pub fn sort_dirs_up(&self) -> Self {
//         let mut entries = self.0.clone();
//         entries.sort();
//         Self(entries)
//     }
//     pub fn sort_files_up(&self) -> Self {
//         let mut entries = self.0.clone();
//         entries.sort();
//         entries.reverse();
//         Self(entries)
//     }
//     pub fn to_output(&self) -> Output {
//         Output::new(self)
//     }
//     pub fn get_permissions(&self, path: &str) -> Vec<std::fs::Permissions> {
//         self.0
//             .iter()
//             .map(|f| {
//                 std::fs::File::open(format!("{}/{}", path, f.file_name()))
//                     .expect("error get permissions")
//                     .metadata()
//                     .unwrap()
//                     .permissions()
//             })
//             .collect()
//     }
// }
// pub struct Output(Vec<(String, String)>);
// impl Output {
//     pub fn new(entries: &Entries) -> Self {
//         Self(
//             entries
//                 .0
//                 .iter()
//                 .map(|f| {
//                     let c = f.to_string();
//                     let re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
//                     (re.replace_all(c.as_str(), "").to_lowercase(), c.clone())
//                 })
//                 .collect(),
//         )
//     }
//     pub fn default_sort(&self) -> Self {
//         let mut vec = self.0.clone();
//         vec.sort_by(|a, b| a.0.cmp(&b.0));
//         Self(vec)
//     }
//     pub fn add_permissions(&self, perms: Vec<std::fs::Permissions>) -> Self {
//         let std_perms = perms.iter().map(|p| {
//             let mut s = String::new();
//             s.push(if p.mode() & 0o100 == 0o100 { 'r' } else { '-' });
//             s.push(if p.mode() & 0o010 == 0o010 { 'w' } else { '-' });
//             s.push(if p.mode() & 0o001 == 0o001 { 'x' } else { '-' });
//             s.push(' ');
//             s.push(if p.mode() & 0o1000 == 0o1000 { 'r' } else { '-' });
//             s.push(if p.mode() & 0o0100 == 0o0100 { 'w' } else { '-' });
//             s.push(if p.mode() & 0o0010 == 0o0010 { 'x' } else { '-' });
//             s.push(if p.mode() & 0o0001 == 0o0001 { 'x' } else { '-' });
//             s
//         });
//         Self(
//             self.0
//                 .iter()
//                 .zip(std_perms)
//                 .map(|(f, p)| (f.0.clone(), format!("{} {}", p, f.1)))
//                 .collect(),
//         )
//     }
//     pub fn to_one_row(&self) -> String {
//         self.0.iter().map(|(_, c)| format!("{} ", c)).collect()
//     }
//     pub fn to_multi_rows(&self) -> String {
//         self.0.iter().map(|(_, c)| format!("{}\n", c)).collect()
//     }
// }
