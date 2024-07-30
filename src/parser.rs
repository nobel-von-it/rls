// pub struct Config {
//     // dir name without command
//     pub dir_name: String,
//     pub add_number: bool,
//     pub multi_line: bool,
//     pub all: bool,
//     pub show_dot: bool,
//     pub dont_sort: bool,
//     pub sort_dir_up: bool,
//     pub sort_file_up: bool,
//     pub no_color: bool,
// }
// impl Default for Config {
//     fn default() -> Self {
//         Self {
//             dir_name: String::from("./"),
//             add_number: false,
//             multi_line: false,
//             all: false,
//             show_dot: false,
//             dont_sort: false,
//             sort_dir_up: false,
//             sort_file_up: false,
//             no_color: false,
//         }
//     }
// }
// impl Config {
//     pub fn parse(flags: Flags) -> Self {
//         let mut config = Config::default();
//         flags.0.iter().for_each(|f| match f {
//             Flag::MultiLine => config.multi_line = true,
//             Flag::AddNumber => config.add_number = true,
//             Flag::All => config.all = true,
//             Flag::ShowDot => config.show_dot = true,
//             Flag::DontSort => config.dont_sort = true,
//             Flag::SortDirUp => config.sort_dir_up = true,
//             Flag::SortFileUp => config.sort_file_up = true,
//             Flag::DirName(s) => config.dir_name = s.clone(),
//             Flag::NoColor => config.no_color = true,
//             Flag::Other => {}
//         });
//         config
//     }
// }
// pub struct Flags(Vec<Flag>);
// impl Flags {
//     pub fn new(args: &[String]) -> Self {
//         Self(args.iter().map(|f| Flag::from(f.to_string())).collect())
//     }
// }
//
// #[derive(Debug, PartialEq, Eq)]
// pub enum Flag {
//     DirName(String),
//     // command -n or --number
//     AddNumber,
//     // command -r or --row
//     MultiLine,
//     // command -a or --all
//     All,
//     // command -d or --dot
//     ShowDot,
//     // command -ds or --dont-sort
//     DontSort,
//     // command -sd or --sort-dir
//     SortDirUp,
//     // command -sf or --sort-file
//     SortFileUp,
//     // commnad -nc or --no-color
//     NoColor,
//     // something else (not supporting)
//     Other,
// }
// impl From<String> for Flag {
//     fn from(s: String) -> Self {
//         // parse other flags
//         if !s.contains("-") {
//             return Flag::DirName(s);
//         }
//         match s.as_str() {
//             "-n" | "--number" => Flag::AddNumber,
//             "-r" | "--row" => Flag::MultiLine,
//             "-a" | "--all" => Flag::All,
//             "-d" | "--dot" => Flag::ShowDot,
//             "-sd" | "--sort-dir" => Flag::SortDirUp,
//             "-sf" | "--sort-file" => Flag::SortFileUp,
//             "-ds" | "--dont-sort" => Flag::DontSort,
//             "-nc" | "--no-color" => Flag::NoColor,
//             _ => Flag::Other,
//         }
//     }
// }

pub struct Config {
    pub dir_name: String,
    pub multi_line: bool,
    pub all: bool,
    pub show_dot: bool,
    pub dont_sort: bool,
    pub no_color: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            dir_name: String::from("./"),
            multi_line: false,
            all: false,
            show_dot: false,
            dont_sort: false,
            no_color: false,
        }
    }
}
impl Config {
    pub fn parse(flags: Vec<String>) -> Self {
        let mut config = Config::default();
        flags.iter().for_each(|f| match f.as_str() {
            "-r" | "--row" => config.multi_line = true,
            "-a" | "--all" => config.all = true,
            "-d" | "--dot" => config.show_dot = true,
            "-ds" | "--dont-sort" => config.dont_sort = true,
            "-nc" | "--no-color" => config.no_color = true,
            _ => if !f.contains("-") {
                config.dir_name = f.clone();
            }
        });
        config
    }
}