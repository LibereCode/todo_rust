// TODO: ???
// Should this instead be split into: `parser.rs` and `colorizer.rs`
//      `parser.rs` => parse strings (entries) to a struct(?)
//      `colorizer.rs` => take parser and colorize segments

use crate::colors::Colorize;

struct PriorityColor {
    a: &'static str,
    b: &'static str,
    c: &'static str,
}

const PRIO_COLOR: PriorityColor = PriorityColor {
    a: "red",
    b: "yellow",
    c: "green",
};

pub trait Parse {
    fn parse_todo(&self) -> String;
}

impl Parse for str {
    fn parse_todo(&self) -> String {
        if self.starts_with("+ ") {
            return self.color("grey");
        }

        if !self.starts_with('(') {
            return self.to_owned();
        }

        let mut chars = self.chars();
        chars.next(); // skip '('

        let Some(prio) = chars.next() else {
            return self.to_owned();
        };

        if chars.next() != Some(')') {
            return self.to_owned();
        }
        let color = match prio {
            'A' => PRIO_COLOR.a,
            'B' => PRIO_COLOR.b,
            'C' => PRIO_COLOR.c,
            _ => return self.to_owned(),
        };

        let remaining: String = chars.into_iter().collect();
        let prio_part_pre: String = ['(', prio, ')'].into_iter().collect();
        let prio_part = prio_part_pre.color(color);
        format!("{}{}", prio_part, remaining)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prio_parsed() {
        assert_eq!("(A) Prio A".parse_todo(), "\x1b[31m(A)\x1b[0m Prio A");
        assert_eq!("(B) Prio B".parse_todo(), "\x1b[33m(B)\x1b[0m Prio B");
        assert_eq!("(C) Prio C".parse_todo(), "\x1b[32m(C)\x1b[0m Prio C");
        assert_eq!("(D) TODO...".parse_todo(), "(D) TODO...");
        assert_eq!("+ (A) WARN".parse_todo(), "\x1b[90m+ (A) WARN\x1b[0m");
        assert_eq!("(A )Bad format?".parse_todo(), "(A )Bad format?");
    }
}
