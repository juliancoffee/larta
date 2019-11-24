use std::process::Command;

pub fn python() -> String {
    let output = Command::new("python")
        .arg("-V")
        .output()
        .expect("failed to get version of python");

    String::from_utf8(output.stdout).unwrap().trim().to_string()
}


