use std::env;

pub struct Options {
    pub is_short: bool,
    pub level: u32,
}

pub fn options() -> Options {
    let args = env::args();
    parse_args(args)
}

fn parse_args(args: env::Args) -> Options {
    let mut is_short = false;
    let mut level_placeholder = "";
    let argv: Vec<String> = args.collect();
    let args = argv.iter();

    for (i, arg) in args.enumerate() {
        match arg.as_str() {
            "-s" | "--short" => {
                is_short = true;
            }
            "-l" | "--level" => {
                level_placeholder = match argv.get(i + 1) {
                    Some(n) => n,
                    None => "",
                };
            }
            _ => (),
        }
    }
    let level: u32 = if level_placeholder == "" {
        0
    } else {
        if let Ok(n) = level_placeholder.parse() {
            n
        } else {
            0
        }
    };
    Options {
        is_short: is_short,
        level: level,
    }
}
