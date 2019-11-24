pub struct Color {
    red: String,
    green: String,
    blue: String,
}

pub fn colorized(content: String, code: Color) -> String {
    let red = code.red;
    let green = code.green;
    let blue = code.blue;

    format!(
        "\x1b[38;2;{r};{g};{b}m{s}\x1b[0m",
        r = red,
        g = green,
        b = blue,
        s = content
    )
}

pub fn magenta(content: String) -> String {
    let code = Color {
        red: String::from("231"),
        green: String::from("0"),
        blue: String::from("247"),
    };

    colorized(content, code)
}

pub fn green(content: String) -> String {
    let code = Color {
        red: String::from("0"),
        green: String::from("250"),
        blue: String::from("0"),
    };

    colorized(content, code)
}
