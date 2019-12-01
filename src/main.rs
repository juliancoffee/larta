extern crate dirs;

mod pwd;
mod arg;
mod colors;
mod version;

fn main() {

    let options = arg::options();
    let need_short = options.is_short;
    let pwd = pwd::current_path(need_short);

    let py_version = version::python();

    if options.level == 1 {
        println!(
            "{}", colors::magenta(String::from(" ~> ")) 
            )
    } else if options.level == 2 {
        println!("on {} with {}",
                 colors::magenta(pwd),
                 colors::green(py_version)
                 );
    } else {
    println!(
        "on {} with {}\n{}",
        colors::magenta(pwd),
        colors::green(py_version),
        colors::magenta(String::from(" ~> ")),
        );
    }
}
