use std::path;
use std::env;
extern crate dirs;

fn home() -> String {
    let home = dirs::home_dir().expect("failed to get home directory");
    home.to_str().expect("failed to parse home directore").to_string()
}

pub fn current_path(need_short: bool) -> String {
    let cur_dir = env::current_dir().expect("failed to get current directory");
    
    if need_short {
        short_pwd(cur_dir)
    } else {
        pwd(cur_dir)
    }
}

fn pwd(current_dir: path::PathBuf) -> String {
    current_dir.to_str().expect("failed to parse current path").to_string()
}

fn short_pwd(current_dir: path::PathBuf) -> String {
    let dir_sep = path::MAIN_SEPARATOR;

    let head = current_dir.file_name().expect("failed to get name of directory")
        .to_str().expect("failed to parse name of directory");

    let tail = current_dir.parent().expect("failed to get parrent path");
    let tail = tail.to_str().expect("failed to parse parrent path");
    let tail = tail.replacen(&home(), "~", 1);

    let mut short_path: Vec<String> = tail.split(dir_sep)
        .map(|s| format!("{}", s.chars().nth(0)
                         .expect("failed to get first character of str")))
        .collect();
    short_path.push(String::from(head));

    let sep: &str = &[dir_sep].iter().collect::<String>();
    short_path.join(sep)
}
