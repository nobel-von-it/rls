pub struct Config {
    pub dir_name: String,
    pub multi_line: bool,
    pub all: bool,
    pub show_dot: bool,
    pub path: bool,
    pub dont_sort: bool,
    pub no_color: bool,
    pub recursive: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            dir_name: String::from("./"),
            multi_line: false,
            all: false,
            show_dot: false,
            path: false,
            dont_sort: false,
            no_color: false,
            recursive: false,
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
            "-p" | "--path" => config.path = true,
            "-ds" | "--dont-sort" => config.dont_sort = true,
            "-nc" | "--no-color" => config.no_color = true,
            "-R" | "--recursive" => config.recursive = true,
            _ => {
                if !f.contains("-") {
                    config.dir_name = f.clone();
                }
            }
        });
        config
    }
}
