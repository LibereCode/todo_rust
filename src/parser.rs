// TODO: ???
// Changed my mind
// Parse here and return 2 things:
//      1. Parsed String
//      2. meta-table ??
// Parse by taking input string, split into words (by whitespace),
// then for each word split into chars (`word.as_byte()`),
// then do simple logic comparing.

use crate::colors::Colorize;
use chrono::NaiveDate;

#[derive(Debug, PartialEq, Clone)]
pub enum Priority {
    A,
    B,
    C,
    D,
    Done,
    Other,
    None,
}

#[derive(Debug, Clone)]
pub enum BodyPart {
    Prio(String),
    Date(NaiveDate),
    Project(String),
    Context(String),
    Normal(String),
}

#[derive(Debug, Clone)]
/// # TODO:
/// - [ ] Use MetaTable.date
/// - [ ] Use MetaTable.projects
/// - [ ] Use MetaTable.contexts
pub struct MetaTable {
    pub prio: Priority,
    pub date: Option<NaiveDate>,
    // pub projects: Vec<String>,
    // pub contexts: Vec<String>,
    pub body: Vec<BodyPart>,
}

struct PriorityColor {
    a: &'static str,
    b: &'static str,
    c: &'static str,
    d: &'static str,
    done: &'static str,
    other: &'static str,
}

const PRIO_COLOR: PriorityColor = PriorityColor {
    a: "red",
    b: "yellow",
    c: "green",
    d: "cyan",
    done: "grey",
    other: "magenta",
};

const PROJECT_COLOR: &str = "green2";
const CONTEXT_COLOR: &str = "cyan";
const DATE_COLOR: &str = "grey";

pub trait ParseStr {
    fn parse2metatable(&self) -> MetaTable;
}

pub trait ParseMetaTable {
    fn colorize(&self) -> String;
    fn to_raw(&self) -> String;
}

impl ParseStr for str {
    fn parse2metatable(&self) -> MetaTable {
        let mut body: Vec<BodyPart> = Vec::new();
        let mut prio = Priority::None;
        let mut date: Option<NaiveDate> = None;
        let mut contexts: Vec<String> = Vec::new();
        let mut projects: Vec<String> = Vec::new();

        let words = self.split_whitespace();
        for (index, word) in words.clone().enumerate() {
            let bytes = word.as_bytes();
            let first = bytes[0] as char;
            let last = bytes[bytes.len() - 1] as char;

            let mut push_me: BodyPart;

            if index == 0 {
                if word == "x" {
                    // body = Vec::from_iter(words.map(str::to_string).map(BodyPart::Normal));
                    body.push(BodyPart::Prio(self.to_string()));
                    prio = Priority::Done;
                    break;
                } else if first == '(' && word.len() == 3 && last == ')' {
                    let prio_char = bytes[1] as char;
                    prio = match prio_char {
                        'A' => Priority::A,
                        'B' => Priority::B,
                        'C' => Priority::C,
                        'D' => Priority::D,
                        _ => Priority::Other,
                    };
                    push_me = BodyPart::Prio(word.to_string());
                } else {
                    push_me = BodyPart::Normal(word.to_string());
                    date = NaiveDate::parse_from_str(word, "%Y-%m-%d")
                        .ok()
                        .inspect(|val| push_me = BodyPart::Date(*val));
                }
            } else if first == '+' {
                push_me = BodyPart::Project(word.to_string());
                projects.push(word.to_string());
            } else if first == '@' {
                push_me = BodyPart::Context(word.to_string());
                contexts.push(word.to_string());
            } else {
                push_me = BodyPart::Normal(word.to_string());
                if index == 1 && prio != Priority::None {
                    date = NaiveDate::parse_from_str(word, "%Y-%m-%d")
                        .ok()
                        .inspect(|val| push_me = BodyPart::Date(*val));
                }
            }

            body.push(push_me);
        }

        MetaTable {
            prio,
            date,
            // projects,
            // contexts,
            body,
        }
    }
}

impl ParseMetaTable for MetaTable {
    /// _Colorize_ (and _parse_) `&self`
    fn colorize(&self) -> String {
        use BodyPart::{Context, Date, Normal, Project};
        use Priority::{A, B, C, D, Done};

        let mut parsed: Vec<String> = Vec::new();

        for word in &self.body {
            //TODO: Match from metatable instead of parse directly
            match word {
                BodyPart::Prio(prio) => {
                    let color = match self.prio {
                        A => PRIO_COLOR.a,
                        B => PRIO_COLOR.b,
                        C => PRIO_COLOR.c,
                        D => PRIO_COLOR.d,
                        Done => PRIO_COLOR.done,
                        _ => PRIO_COLOR.other,
                    };
                    parsed.push(prio.color(color));
                }
                Date(date) => parsed.push(date.to_string().color(DATE_COLOR)),
                Project(proj) => parsed.push(proj.color(PROJECT_COLOR)),
                Context(cont) => parsed.push(cont.color(CONTEXT_COLOR)),
                Normal(norm) => parsed.push(norm.to_owned()),
                // _ => {}
            }
        }

        parsed.join(" ")
    }

    fn to_raw(&self) -> String {
        use BodyPart::{Context, Date, Normal, Prio, Project};
        let mut raw: Vec<String> = Vec::new();
        for word in &self.body {
            match word {
                Date(date) => raw.push(date.to_string()),
                Prio(s) | Project(s) | Context(s) | Normal(s) => raw.push(s.to_owned()),
            }
        }
        raw.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prio_parsed() {
        assert_eq!(
            "(A) Prio A".parse2metatable().colorize(),
            "\x1b[31m(A)\x1b[0m Prio A"
        );
        assert_eq!(
            "(B) Prio B".parse2metatable().colorize(),
            "\x1b[33m(B)\x1b[0m Prio B"
        );
        assert_eq!(
            "(C) Prio C".parse2metatable().colorize(),
            "\x1b[32m(C)\x1b[0m Prio C"
        );
        assert_eq!(
            "(D) Prio D".parse2metatable().colorize(),
            "\x1b[36m(D)\x1b[0m Prio D"
        );
        assert_eq!(
            "(E) Other".parse2metatable().colorize(),
            "\x1b[35m(E)\x1b[0m Other"
        );
        assert_eq!(
            "x (A) WARN".parse2metatable().colorize(),
            "\x1b[90mx (A) WARN\x1b[0m"
        );
        assert_eq!(
            "(A )Bad format?".parse2metatable().colorize(),
            "(A )Bad format?"
        );
    }

    // TODO:
    // #[test]
    // fn project_parsed() {
    //     assert_eq!(
    //         "(A) Prio A".parse2metatable().parse_todo(),
    //         "\x1b[31m(A)\x1b[0m Prio A"
    //     );
    // }

    // TODO:
    // #[test]
    // fn context_parsed() {
    //     assert_eq!(
    //         "(A) Prio A".parse2metatable().parse_todo(),
    //         "\x1b[31m(A)\x1b[0m Prio A"
    //     );
    // }

    // TODO:
    // #[test]
    // fn date_parsed() {
    //     assert_eq!(
    //         "(A) Prio A".parse2metatable().parse_todo(),
    //         "\x1b[31m(A)\x1b[0m Prio A"
    //     );
    // }

    // TODO:
    // #[test]
    // fn all_parsed() {
    //     assert_eq!(
    //         "(A) Prio A".parse2metatable().parse_todo(),
    //         "\x1b[31m(A)\x1b[0m Prio A"
    //     );
    // }
}
