use std::env;
use std::io;
use std::io::{Write};
use std::path::{Path, PathBuf, Component};

enum TruncateError {
    EnvHomeNotFound,
}

fn truncate_path(pb: PathBuf, trunc: usize) -> Result<PathBuf, TruncateError> {
    let home =
        if let Ok(home_str) = env::var("HOME") {
            PathBuf::from(home_str)
        } else {
            return Err(TruncateError::EnvHomeNotFound);
        };

    let path =
        if pb == home {
            return Ok(PathBuf::from("~"));
        } else if pb.starts_with(&home) {
            Path::new("~").join(pb.strip_prefix(home).unwrap())
        } else {
            pb
        };

    let path_len = path.iter().count();
    let mut components = path.components();

    if (path_len <= trunc + 1 && components.next().unwrap() == Component::RootDir)
       || path_len <= trunc {
        Ok(path)
    } else {
        Ok(path.iter().skip(path_len - trunc).collect())
    }
}

fn print_path(p: &str) {
    print!("{}/", p);
    io::stdout().flush().unwrap();
}

fn usage() {
    println!("usage: home [show (default 2)]");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let trunc =
        match args.get(1) {
            Some(arg) => {
                if let Ok(val) = arg.parse::<usize>() {
                    val
                } else {
                    println!("error: cannot parse \"{}\" as a number", arg);
                    usage();
                    print_path("---");
                    return;
                }
            },
            None => 2
        };

    let pwd = env::current_dir().unwrap();

    match truncate_path(pwd, trunc) {
        Ok(pb) => print_path(pb.to_str().unwrap()),
        Err(err) => match err {
            TruncateError::EnvHomeNotFound => {
                println!("env var: $HOME not found - ");
                print_path("---");
            }
        }
    }
}
