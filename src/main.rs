use std::env;

extern crate dirs;
mod arg;
mod colors;
mod version;

fn main() {
    let home = dirs::home_dir().expect("failed to get home directory");
    let home = home.to_str().unwrap().to_string();

    let curdir = env::current_dir().expect("failed to get current directory");
    let curdir = curdir.to_str().unwrap().to_string();

    let dir_sep = if cfg!(windows) {
        "\\" 
    } else {
        "/"
    };

    let pwd = curdir.replacen(&home, "~", 1);
    let short_pwd: Vec<String> = pwd.split(dir_sep)
        .map(|s| format!("{}", s.chars().nth(0).unwrap()))
        .collect();
    let pwd = if arg::line(env::args()).is_short {
        short_pwd.join(dir_sep) 
    } else {
        pwd
    };

    println!(
        "on {} with {}\n{}",
        colors::magenta(pwd),
        colors::green(version::python()),
        colors::magenta(String::from(" ~> ")),
    );
}
