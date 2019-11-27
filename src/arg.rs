use std::env;

pub struct Options {
    pub is_short: bool,
}

pub fn options() -> Options {
    let args = env::args();
    parse_args(args)
}

fn parse_args(args: env::Args) -> Options {
    let mut is_short = false;
    //let args: Vec<&str> = args
    //  .map(|s| s.as_str()).collect();
    for arg in args {
        match arg.as_str() {
            "-s" => {
                is_short = true;
            }
            "--short" => {
                is_short = true;
            }
            _ => (),
        }
    }
    Options { is_short: is_short }
}

