use std::path;
use std::env;

extern crate dirs;

pub fn current_path(need_short: bool) -> String {
    let cur_dir = env::current_dir().expect("failed to get current directory");
    
    if need_short {
        short_pwd(cur_dir)
    } else {
        pwd(cur_dir)
    }
}

fn pwd(current_dir: path::PathBuf) -> String {
    let path = current_dir.to_str().expect("failed to parse current path").to_string();
    let home = home();
    let home = home.to_str().expect("failed to parse home directory");
    path.replacen(&home, "~", 1)
}

fn short_pwd(current_dir: path::PathBuf) -> String {
    if current_dir == home() {
        return String::from("~")
    }

    let home = home();
    let home = home.to_str().expect("failed to parse home directory");
    let dir_sep = path::MAIN_SEPARATOR;
    let sep: &str = &[dir_sep].iter().collect::<String>();

    let head = match current_dir.file_name() {
        Some(name) => name,
        None => path::Path::new("").as_os_str()
    }.to_str().expect("failed to parse name of directory");

    let tail = match current_dir.parent() {
        Some(name) => name,
        None => path::Path::new("")
    }.to_str().expect("failed to parse parrent path");
        //.expect("failed to get parrent path");
    //let tail = parent_path.to_str().expect("failed to parse parrent path");

    let tail = tail.replacen(&home, "~", 1);

    let mut short_path: Vec<String> = tail.split(dir_sep)
        .map(|s| match s {
            "" => String::from(""),
            _ => format!("{}", s.chars().nth(0)
                         .expect("failed to get first character of str"))
        })
        .collect();
    if tail == String::from("/") {
        format!("/{}", head)
    } else {
        short_path.push(String::from(head));
        short_path.join(&*sep)
    }

}

fn home() -> path::PathBuf {
    dirs::home_dir().expect("failed to get home directory")
}
