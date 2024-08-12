use files::Entries;
use parser::Config;

use crate::files::Output;

// const USAGE: &str = "Usage: ls [OPTION]... [FILE]...\n\
// List information about the FILEs (the current directory by default).\n\
// Sort entries alphabetically if none of -cftuvSUX nor --sort is specified.\n\
// \n\
// Mandatory arguments to long options are mandatory for short options too.\n\
// \n\
//   -a, --all                  do not ignore entries starting with .\n\
//   -A, --almost-all           do not list implied . and ..\n\
//   -b, --escape               print C-style escapes for nongraphic characters\n\
//   -B, --ignore-backups        do not list implied entries ending with ~\n\
//   -c                         with -lt: sort by, and show, ctime (time of last modification of file status)\n\
//   -C                         list files in columns\n\
//   --color[=WHEN]             control whether color is used to distinguish file types\n\
//   -d, --directory            list directory entries instead of contents, and do not dereference symbolic links\n\
//   -D, --dired                generate output designed for Emacs' dired mode\n\
//   -f                         do not sort, enable -aU, disable -ls --color\n\
//   -F, --classify             append indicator (one of */=>@|) to entries\n\
//   --file-type               likewise, except do not append '*'\n\
//   --format=WORD             across -x, commas -m, horizontal -x, long -l, single-column -1, verbose -l, vertical -C\n\
//   --full-time                 like -l --format=full-time\n\
//   -g                         like -l, but do not list owner\n\
//   --group-directories-first  group directories before files;\n\
//                               can be augmented with a --sort option, but any use of --sort=-none (-n) disables grouping\n\
//   -G, --no-group             in a long listing, don't print group names\n\
//   -h, --human-readable       with -l and/or -s, print human-readable sizes (e.g., 1K 234M 2G)\n\
//   --si                       likewise, but use powers of 1000 not 1024\n\
//   -H, --dereference-command  follow symbolic links listed on the command line\n\
//   --indicator-style=WORD     appendix indicator (none, slash (-p), file-type (--file-type), classify (-F))\n\
//   -i, --inode               print the index number of each file\n\
//   -I, --ignore=PATTERN      do not list files whose names match the PATTERN\n\
//   -l                         use a long listing format\n\
//   -L, --dereference         when showing file information for a symbolic\n\
//                               link, show information for the file the link references rather than the link itself\n\
//   -m                         fill width with a comma separated list of entries\n\
//   -n, --numeric-uid-gid     like -l, but list numeric user and group IDs\n\
//   -o                         like -l, but do not list group information\n\
//   -p, --indicator-style=slash\n\
//                               append / indicator to directories\n\
//   -q, --hide-control-chars  print ? instead of non graphic characters\n\
//   --show-control-chars      show non graphic characters as-is (the default, unless program is 'ls' and output is a terminal)\n\
//   --quoting-style=WORD      use quoting style WORD for entry names:\n\
//                               escape, literal, locale, shell, shell-always, shell-escape, c, escape\n\
//   -r, --reverse             reverse order while sorting\n\
//   -R, --recursive          list subdirectories recursively\n\
//   -s, --size               print the allocated size of each file, in blocks\n\
//   -S                         sort by file size, largest first\n\
//   --sort=WORD               sort by WORD instead of name: none (-U), size (-S), time (-t), version (-v), extension (-X)\n\
//   --time=WORD               with -l, show time as WORD instead of default modification time:\n\
//                               atime, access, use, ctime or status; use specified time as sort time if --sort=time\n\
//   -t                         sort by modification time, newest first\n\
//   -u                         with -lt: sort by, and show, access time;\n\
//                               with -l: show access time and sort by name;\n\
//                               otherwise: sort by access time, newest first\n\
//   -U                         do not sort; list entries in directory order\n\
//   -v                         natural sort of (version) numbers within text\n\
//   -w, --width=N             assume screen width instead of current value\n\
//   -x                         list entries by lines instead of by columns\n\
//   -X                         sort alphabetically by entry extension\n\
//   --help     display this help and exit\n\
//   --version  output version information and exit";

mod files;
mod parser;

fn main() {
    let config = Config::parse(std::env::args().skip(1).collect::<Vec<String>>());
    let entries = Entries::new(&config.dir_name);
    if config.all {
        println!(
            "{}",
            Output::new_fn(&entries, |f| f.to_string()).show_multiple_rows()
        );
    } else {
        let output = if config.show_dot {
            Output::new_fn(&entries, |f| f.colored_file_name())
        } else {
            Output::new_hide_dots(&entries)
        };
        if config.multi_line {
            println!("{}", output.show_multiple_rows());
        } else {
            println!("{}", output.show_single_row());
        }
    }
}
