// TODO: ???
// Changed my mind
// Parse here and return 2 things:
//      1. Parsed String
//      2. meta-table ??
// Parse by taking input string, split into words (by whitespace),
// then for each word split into chars (`word.as_byte()`),
// then do simple logic comparing.

use crate::colors::Colorize;

// TODO: put metatable in another module?
//
// #[derive(Debug)]
// pub struct MetaTable {
//     pub entries: Vec<usize>,
//     pub prio_a: Vec<usize>,
//     pub prio_b: Vec<usize>,
//     pub prio_c: Vec<usize>,
//     pub prio_d: Vec<usize>,
//     pub prio_else: Vec<usize>,
//     pub projects: Vec<usize>,
//     pub context: Vec<usize>,
// }
// #[derive(Debug)]
// pub struct ParseResult {
//     pub results: Vec<String>,
//     pub meta: MetaTable,
// }

struct PriorityColor {
    a: &'static str,
    b: &'static str,
    c: &'static str,
    d: &'static str,
    other: &'static str,
}

const PRIO_COLOR: PriorityColor = PriorityColor {
    a: "red",
    b: "yellow",
    c: "green",
    d: "cyan",
    other: "magenta",
};

const PROJECT_COLOR: &'static str = "green2";
const CONTEXT_COLOR: &'static str = "cyan";
const DONE_COLOR: &'static str = "grey";

pub trait Parse {
    fn parse_todo(&self) -> String;
}

impl Parse for str {
    fn parse_todo(&self) -> String {
        let mut parsed: Vec<String> = Vec::new();

        for (index, word) in self.split_whitespace().enumerate() {
            let bytes = word.as_bytes();
            let first = bytes[0] as char;
            let last = bytes[bytes.len() - 1] as char;

            if index == 0 {
                if word == "+" {
                    parsed.push(self.color(DONE_COLOR));
                    break;
                } else if first == '(' && word.len() == 3 && last == ')' {
                    let prio = bytes[1] as char;
                    let color = match prio {
                        'A' => PRIO_COLOR.a,
                        'B' => PRIO_COLOR.b,
                        'C' => PRIO_COLOR.c,
                        'D' => PRIO_COLOR.d,
                        _ => PRIO_COLOR.other,
                    };
                    parsed.push(word.color(color));
                } else {
                    parsed.push(word.to_string());
                }
            } else if first == '+' {
                parsed.push(word.color(PROJECT_COLOR))
            } else if first == '@' {
                parsed.push(word.color(CONTEXT_COLOR));
            } else {
                parsed.push(word.to_string());
            }
        }

        parsed.join(" ")

        // if self.starts_with("+ ") {
        //     return self.color("grey");
        // }
        //
        // if !self.starts_with('(') {
        //     return self.to_owned();
        // }
        //
        // let mut chars = self.chars();
        // chars.next(); // skip '('
        //
        // let Some(prio) = chars.next() else {
        //     return self.to_owned();
        // };
        //
        // if chars.next() != Some(')') {
        //     return self.to_owned();
        // }
        // let color = match prio {
        //     'A' => PRIO_COLOR.a,
        //     'B' => PRIO_COLOR.b,
        //     'C' => PRIO_COLOR.c,
        //     _ => return self.to_owned(),
        // };
        //
        // let remaining: String = chars.into_iter().collect();
        // let prio_part_pre: String = ['(', prio, ')'].into_iter().collect();
        // let prio_part = prio_part_pre.color(color);
        // format!("{}{}", prio_part, remaining)
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
