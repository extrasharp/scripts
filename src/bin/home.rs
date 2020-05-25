use std::env;
use std::path::{Path, PathBuf};

// todo clean up

fn main() {
    let trunc = 2;

    let pwd = env::current_dir().unwrap();
    let home =
        if let Ok(path) = env::var("HOME") {
            PathBuf::from(path)
        } else {
            print!("env vae: $HOME not found - ");
            return;
        };

    if pwd == home {
        print!("~/");
        return;
    }

    let path =
        if pwd.starts_with(&home) {
            Path::new("~").join(pwd.strip_prefix(&home).unwrap())
        } else {
            pwd
        };

    let path_len = path.iter().count();
    let trunced: PathBuf =
        if path_len > trunc {
            let mut iter = path.iter().peekable();
            let trunc =
                if *iter.peek().unwrap() == std::ffi::OsStr::new("/") {
                    trunc + 1
                } else {
                    trunc
                };
            path.iter().skip(path_len - trunc).collect()
        } else {
            path
        };

    print!("{}/", trunced.to_str().unwrap());
    // io::stdout.flush()
}
