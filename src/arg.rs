pub struct Line {
    pub is_short: bool
}

pub fn line(args: std::env::Args) -> Line {
    let mut is_short = false;
    //let args: Vec<&str> = args
      //  .map(|s| s.as_str()).collect();
    for arg in args {
        match arg.as_str() {
            "-s" => {
                is_short = true;
            },
            "--short" => {
                is_short = true;
            },
            _ => (),
        }
    }
    Line {
        is_short: is_short
    }
}
