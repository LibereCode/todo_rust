mod argparse;
use crate::argparse::{Commands, Flags, ParsedArgs};

mod colors;
use colors::Colorize;

mod live_input;
use live_input::read_input_live;

mod parser;
use parser::Parse;

use std::{
    collections::HashMap,
    fs::{self},
    io::{self, Write},
    path::Path,
    process::exit,
};

/// # TODO
/// move this to `config()`
const TODO_PATH: &str = "out.todo.txt";

const HELP: &str = "
todo_rust — read/write todo.txt. rust btw.

Usage:
    todo_rust [OPTIONS]

Arguments:
    -f, --file <FILE>    Target file
    -h, --help           Print this help
    -v, --verbose        Be more verbose
    -V, --version        Print version

Commands:
    add [ENTRY]          Add an entry. Leave empty to get an input prompt.
    done [INDEX]         Mark an entry as done by index. Leave empty to to get an input prompt.
    ls                   List added entries (read todo file).
    rm [INDEX]           Remove an entry by index. Leave empty to to get an input prompt.
";

/// `version = [major, minor, micro];`
const VERSION: [u8; 3] = [0, 0, 1];

// /// # TODO
// /// - [ ] Read env-var or config file
// /// - [ ] Default values
// fn config() {
// }
//
// /// TODO
// #[derive(Debug)]
// enum ConfigValue {
//     String(String),
//     Integer(i64),
//     Boolean(bool),
// }
/// TODO
struct Config {
    values: HashMap<String, String /*ConfigValue*/>,
    // values: HashMap<String, ConfigValue>, // TODO
}
impl Config {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    fn set(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), value.to_string());
    }
    fn get(&mut self, key: &str) -> Option<&String> {
        self.values.get(key)
    }
}

// /// # TODO
// /// - [ ] Implement `argparse`: reading --flags and commands (see HELP)
// fn argparse() {
// }

/// # TODO
/// - [ ] Minimize number of `?` used
fn ensure_parent(path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

/// # TODO
/// - [ ] Minimize number of `?` used
fn try_write(path: &Path, data: &str) -> io::Result<()> {
    ensure_parent(path)?;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    let data_trim = data.trim_end();
    writeln!(file, "{data_trim}")?;

    Ok(())
}

/// # TODO
/// - [ ] I dunno, something good?
///   At least do not just print shii
/// - Arguments
///   - [ ] -f, --file
///   - [x] -h, --help
///   - [ ] -v, --verbose
///   - [x] -V, --version
/// - Commands
///   - [ ] add [ENTRY]
///   - [ ] done [INDEX]
///   - [ ] ls
///   - [ ] rm [INDEX]
fn interpret_args(cfg: &mut Config) {
    // NOTE temporary
    for i in argparse::parse().unwrap() {
        match i {
            ParsedArgs::Command(cmd) => match cmd {
                Commands::Add(string) => println!("Cmd: Adding; + arg: {string} -- TODO!()"), // TODO
                Commands::Done(nr) => println!("Cmd: Doning; + arg: {nr} -- TODO!()"), // TODO
                Commands::Ls => println!("Cmd: Listing -- TODO!()"),                   // TODO
                Commands::Rm(nr) => println!("Cmd: Removing; + arg: {nr} -- TODO!()"), // TODO
            },
            ParsedArgs::Flag(flag) => match flag {
                Flags::File(file) => {
                    cfg.set("file", file.as_str()); // FIXME should need to consume the next arg
                    // println!("Flag: Filing");
                }
                Flags::Help => {
                    println!("{HELP}");
                    exit(0)
                }
                Flags::Verbose => {
                    cfg.set("verbose", "true"); // FIXME should be a boolean
                    // println!("Flag: Verbosing");
                }
                Flags::Version => {
                    let version = VERSION.map(|i| i.to_string()).join(".");
                    println!("v{version}");
                    exit(0)
                }
            },
        }
    }
}

/// # TODO
/// - [ ] Minimize number of `?` used
fn main() -> io::Result<()> {
    let mut cfg = Config::new();
    cfg.set("file", TODO_PATH);

    interpret_args(&mut cfg);

    println!(
        "{}",
        "Write a todo.txt entry, (or something...)".color("blue")
    );
    let entry: String = read_input_live()?;
    // println!("[DEBUG] todo.txt entry: \"{entry}\"");
    // let entry_parsed: String = entry.parse_todo();
    // println!("[DEBUG] todo.txt entry_parsed: \"{entry_parsed}\"");

    let path = Path::new(cfg.get("file").expect("cfg.file is missing"));

    try_write(path, entry.as_str())?;

    let text = fs::read_to_string(path).unwrap_or_default();
    // println!("DEBUG: text = \"{text}\"");

    // TODO use something like this for the parser
    let /* mut */ lines_vec = text.lines().collect::<Vec<_>>();

    for (index, line) in lines_vec.into_iter().enumerate() {
        // println!("{}. {}", index + 1, line);
        println!("{}. {}", index + 1, line.parse_todo());
    }

    Ok(())
}
