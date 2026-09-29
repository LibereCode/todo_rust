struct Color {
    black: &'static str,
    red: &'static str,
    green: &'static str,
    yellow: &'static str,
    blue: &'static str,
    magenta: &'static str,
    cyan: &'static str,
    white: &'static str,
    grey: &'static str,
    reset: &'static str,
}
const COLOR: Color = Color {
    black: "\x1b[30m",
    red: "\x1b[31m",
    green: "\x1b[32m",
    yellow: "\x1b[33m",
    blue: "\x1b[34m",
    magenta: "\x1b[35m",
    cyan: "\x1b[36m",
    white: "\x1b[37m",
    grey: "\x1b[90m",
    reset: "\x1b[0m",
};

pub trait Colorize {
    fn color(&self, color: &str) -> String;
}
impl Colorize for str {
    fn color(&self, color: &str) -> String {
        let code = match color {
            "black" => COLOR.black,
            "red" => COLOR.red,
            "green" => COLOR.green,
            "yellow" => COLOR.yellow,
            "blue" => COLOR.blue,
            "magenta" => COLOR.magenta,
            "cyan" => COLOR.cyan,
            "white" => COLOR.white,
            "grey" => COLOR.grey,
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
        assert_eq!("**R**".color("black"), "\x1b[30m**R**\x1b[0m");
        assert_eq!("**A**".color("red"), "\x1b[31m**A**\x1b[0m");
        assert_eq!("**I**".color("green"), "\x1b[32m**I**\x1b[0m");
        assert_eq!("**N**".color("yellow"), "\x1b[33m**N**\x1b[0m");
        assert_eq!("**B**".color("blue"), "\x1b[34m**B**\x1b[0m");
        assert_eq!("**O**".color("magenta"), "\x1b[35m**O**\x1b[0m");
        assert_eq!("**W**".color("cyan"), "\x1b[36m**W**\x1b[0m");
        assert_eq!("**!**".color("white"), "\x1b[37m**!**\x1b[0m");
        assert_eq!("**?**".color("grey"), "\x1b[90m**?**\x1b[0m");
    }
}
