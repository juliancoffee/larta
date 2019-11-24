use std::env;

extern crate dirs;
mod colors;
mod version;

fn main() {
    let home = dirs::home_dir().expect("failed to get home directory");
    let home = home.to_str().unwrap().to_string();

    let curdir = env::current_dir().expect("failed to get current directory");
    let curdir = curdir.to_str().unwrap().to_string();

    let pwd = curdir.replacen(&home, "~", 1);

    println!(
        "on {} with {}\n{}",
        colors::magenta(pwd),
        colors::green(version::python()),
        colors::magenta(String::from(" ~>")),
    );
}
