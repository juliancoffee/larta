extern crate dirs;

fn main() {
    let home_dir = dirs::home_dir()
        .expect("failed to get home directory");
    let home_str = home_dir.to_str()
        .expect("failed to parse path to string");
    println!("Home: {}", home_str);
}
