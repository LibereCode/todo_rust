use std::env::{self};

pub enum Commands {
    Add,
    Done,
    Ls,
    Rm,
}
pub enum Flags {
    File,
    Help,
    Verbose,
    Version,
}
pub enum ParsedArgs {
    Command(Commands),
    Flag(Flags),
}
/// # TODO
/// - [ ] Should instead return a `Vec` or `Struct` of each _argument_ to use.
///     `main()` will interpret what to do with those.
/// - [ ] For some Commands/Flags, implement consuming the next arg as a positional argument.
///     ie: `todo_rust --file "foobar"` or `todo_rust done 67`
pub fn parse() -> Vec<ParsedArgs> {
    let mut v: Vec<ParsedArgs> = Vec::new();
    for (i, arg) in env::args().enumerate() {
        if i != 0 {
            let bytes = arg.as_bytes();
            if bytes[0] as char == '-' && bytes[1] as char != '-' {
                v = short_flags(v, &bytes[1..]);
            } else {
                match arg.as_str() {
                    // commands
                    "add" => v.push(ParsedArgs::Command(Commands::Add)), // [ENTRY]
                    "done" => v.push(ParsedArgs::Command(Commands::Done)), // [INDEX]
                    "ls" => v.push(ParsedArgs::Command(Commands::Ls)),
                    "rm" => v.push(ParsedArgs::Command(Commands::Rm)), // [INDEX]
                    // flags
                    "--file" => v.push(ParsedArgs::Flag(Flags::File)),
                    "--help" => v.push(ParsedArgs::Flag(Flags::Help)),
                    "--verbose" => v.push(ParsedArgs::Flag(Flags::Verbose)),
                    "--version" => v.push(ParsedArgs::Flag(Flags::Version)),
                    _ => panic!(
                        "Unknown command/flag.\n\tTODO: Better error handling for long flags / args ?"
                    ),
                };
            };
        };
    }
    v
}

/// shawties is should NOT include the first '-'.
/// # example
/// ```rust
/// let arg = String::from("-abc");
/// let shawties = arg.as_bytes()[1..];
/// let shawties_parsed = short_flags(shawties);
/// ```
fn short_flags(mut v: Vec<ParsedArgs>, shawties: &[u8]) -> Vec<ParsedArgs> {
    for i in shawties {
        let char = *i as char;
        match char {
            'f' => v.push(ParsedArgs::Flag(Flags::File)),
            'h' => v.push(ParsedArgs::Flag(Flags::Help)),
            'v' => v.push(ParsedArgs::Flag(Flags::Verbose)),
            'V' => v.push(ParsedArgs::Flag(Flags::Version)),
            _ => panic!("Unknown short flag.\n\tTODO: Better error handling for short flags ?"),
        }
    }
    v
}
