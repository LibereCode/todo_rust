use std::env::{self};

#[derive(Debug, PartialEq)]
pub enum Commands {
    Add(String),
    Done(usize),
    Ls,
    Rm(usize),
}
#[derive(Debug, PartialEq)]
pub enum Flags {
    File(String),
    Help,
    Verbose,
    Version,
}
#[derive(Debug, PartialEq)]
pub enum ParsedArgs {
    Command(Commands),
    Flag(Flags),
}
/// # TODO
/// - [x] Should instead return a `Vec` or `Struct` of each _argument_ to use.
///   `main()` will interpret what to do with those.
/// - [x] For some Commands/Flags, implement consuming the next arg as a positional argument.
///   ie: `todo_rust --file "foobar"` or `todo_rust done 67`
/// - [x] FIX implement Result<Vec<ParsedArgs>, String> -- See Reference
/// - [ ] FIXME: Commands should be processed AFTER flags !! (+ should only allow 1 command ?)
pub fn parse() -> Result<Vec<ParsedArgs>, String> {
    let mut parsed_vec: Vec<ParsedArgs> = Vec::new();
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        if arg.starts_with('-') && !arg.starts_with("--") {
            short_flags(&mut parsed_vec, &mut args, &arg[1..])?;
            continue;
        }

        match arg.as_str() {
            // commands
            "add" => {
                let entry = args
                    .next()
                    .ok_or_else(|| "command `add` requires argument [ENTRY](string)".to_string())?;
                parsed_vec.push(ParsedArgs::Command(Commands::Add(entry))); // [ENTRY]
            }
            "done" => {
                let value = args.next().ok_or_else(|| {
                    "command `done` requires argument [INDEX](integer)".to_string()
                })?;
                let index = value
                    .parse::<usize>()
                    .map_err(|_| format!("invalid index for `done`: {value}"))?;
                parsed_vec.push(ParsedArgs::Command(Commands::Done(index))); // [INDEX]
            }
            "ls" => {
                parsed_vec.push(ParsedArgs::Command(Commands::Ls));
            }
            "rm" => {
                let value = args
                    .next()
                    .ok_or_else(|| "command `rm` requires argument [INDEX](integer)".to_string())?;
                let index = value
                    .parse::<usize>()
                    .map_err(|_| format!("invalid index for `rm`: {value}"))?;
                parsed_vec.push(ParsedArgs::Command(Commands::Rm(index))); // [INDEX]
            }
            // flags
            "--file" => {
                let entry = args
                    .next()
                    .ok_or_else(|| "flag `--file` requires argument [ENTRY](string)".to_string())?;
                parsed_vec.push(ParsedArgs::Flag(Flags::File(entry))); // [ENTRY]
            }
            "--help" => parsed_vec.push(ParsedArgs::Flag(Flags::Help)),
            "--verbose" => parsed_vec.push(ParsedArgs::Flag(Flags::Verbose)),
            "--version" => parsed_vec.push(ParsedArgs::Flag(Flags::Version)),
            _ => return Err(format!("\t[ERROR]: Unknown command/flag ({arg})")),
        }
    }

    Ok(parsed_vec)
}

/// shawties is should NOT include the first '-'.
/// # example
/// ```rust
/// let mut v: Vec<ParsedArgs> = Vec::new();
/// let mut args = env::args().skip(1);
/// let arg = String::from("-abc");
/// short_flags(&mut parsed_vec, &mut args, &arg[1..])?;
/// ```
fn short_flags(
    parsed_vec: &mut Vec<ParsedArgs>,
    args: &mut impl Iterator<Item = String>,
    flags: &str,
) -> Result<(), String> {
    for flag in flags.chars() {
        match flag {
            'f' => {
                let entry = &args
                    .next()
                    .ok_or_else(|| "flag `--file` requires argument [ENTRY](string)".to_string())?;
                parsed_vec.push(ParsedArgs::Flag(Flags::File(entry.clone()))); // [ENTRY]
            }
            'h' => parsed_vec.push(ParsedArgs::Flag(Flags::Help)),
            'v' => parsed_vec.push(ParsedArgs::Flag(Flags::Verbose)),
            'V' => parsed_vec.push(ParsedArgs::Flag(Flags::Version)),
            _ => return Err(format!("\t[ERROR]: Unknown short flag (-{flag})")),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_flags() {
        let mut parsed_vec: Vec<ParsedArgs> = Vec::new();
        let arg = String::from("-vh");
        let mut args = env::args().skip(1);
        short_flags(&mut parsed_vec, &mut args, &arg[1..]).unwrap();
        assert_eq!(
            parsed_vec,
            vec![
                ParsedArgs::Flag(Flags::Verbose),
                ParsedArgs::Flag(Flags::Help),
            ]
        );
    }
}
