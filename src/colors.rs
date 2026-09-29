struct Color {
    red: &'static str,
    green: &'static str,
    yellow: &'static str,
    blue: &'static str,
    reset: &'static str,
}
const COLOR: Color = Color {
    red: "\x1b[31m",
    green: "\x1b[32m",
    yellow: "\x1b[33m",
    blue: "\x1b[34m",
    reset: "\x1b[0m",
};

pub trait Colorize {
    fn color(&self, color: &str) -> String;
}
impl Colorize for str {
    fn color(&self, color: &str) -> String {
        let code = match color {
            // "black" => COLOR.black,
            "red" => COLOR.red,
            "green" => COLOR.green,
            "yellow" => COLOR.yellow,
            "blue" => COLOR.blue,
            // "magenta" => COLOR.magenta,
            // "cyan" => COLOR.cyan,
            // "white" => COLOR.white,
            _ => panic!("unknown color: {color}"),
        };

        format!("{code}{self}{}", COLOR.reset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_color() {
        assert_eq!("red".color("red"), "\x1b[31mred\x1b[0m");
        assert_eq!("green".color("green"), "\x1b[32mgreen\x1b[0m");
        assert_eq!("yellow".color("yellow"), "\x1b[33myellow\x1b[0m");
        assert_eq!("blue".color("blue"), "\x1b[34mblue\x1b[0m");
    }
}
