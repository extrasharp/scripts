use std::env;
use std::io;
use std::io::{Write};
use std::path::{Path, PathBuf, Component};

fn print_path(p: &str) {
    print!("{}/", p);
    io::stdout().flush().unwrap();
}

fn main() {
    let trunc = 2;

    let pwd = env::current_dir().unwrap();
    if pwd == Path::new("/") {
        print_path("");
        return;
    }

    let home =
        if let Ok(home_str) = env::var("HOME") {
            PathBuf::from(home_str)
        } else {
            print!("env var: $HOME not found - ");
            return;
        };

    let path =
        if pwd == home {
            print_path("~");
            return;
        } else if pwd.starts_with(&home) {
            Path::new("~").join(pwd.strip_prefix(home).unwrap())
        } else {
            pwd
        };

    let path_len = path.iter().count();
    let mut components = path.components();

    let trunced: PathBuf =
        if (path_len <= trunc + 1 && components.next().unwrap() == Component::RootDir)
           || path_len <= trunc {
            path
        } else {
            path.iter().skip(path_len - trunc).collect()
        };

    print_path(trunced.to_str().unwrap());
}
