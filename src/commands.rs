use crate::{
    argparse::{self, Commands, Flags, ParsedArgs},
    colors::Colorize,
    config::Config,
    live_input::read_input_live,
    parser::Parse,
};

use std::{
    env,
    fs::{self},
    io::{self, Write},
    path::Path,
    process::exit,
};

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
///
/// # TODO:
/// - [?] Instead parse `Cargo.toml`
const VERSION: [u8; 3] = [0, 0, 1];

/// # TODO:
/// - [?] Minimize number of `?` used
fn ensure_parent(path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

/// # TODO:
/// - [?] Minimize number of `?` used
fn try_append(path: &Path, data: &str) -> io::Result<()> {
    ensure_parent(path)?;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    let data_trim = data.trim_end();
    writeln!(file, "{data_trim}")?;

    Ok(())
}

fn add(cfg: &Config, data: &str) -> io::Result<()> {
    println!(
        "{}",
        "Write a todo.txt entry, (or something...)".color("blue")
    );
    let path = Path::new(cfg.main.filepath.as_str());
    try_append(path, data)?;

    cfg.main
        .verbose
        .then(|| println!("add: {}", data.parse_todo()));

    Ok(())
}

fn add_interactive(cfg: &Config) -> io::Result<()> {
    println!(
        "{}",
        "Write a todo.txt entry, (or something...)".color("blue")
    );
    let data = read_input_live()?;
    let path = Path::new(cfg.main.filepath.as_str());
    try_append(path, data.as_str())?;

    cfg.main.verbose.then(|| println!("{}", data.parse_todo()));

    Ok(())
}

/// $ `line_nr` = `vector_index` + 1; $
fn done(cfg: &Config, line_nr: usize) -> io::Result<()> {
    let path = Path::new(cfg.main.filepath.as_str());

    ensure_parent(path)?;

    let mut lines: Vec<String> = fs::read_to_string(path)?
        .lines()
        .map(str::to_owned)
        .collect();

    let index = line_nr - 1;
    lines[index].insert_str(0, "x ");

    fs::write(path, lines.join("\n") + "\n")?;

    cfg.main
        .verbose
        .then(|| println!("done({}): {}", line_nr, lines[index].parse_todo()));

    Ok(())
}

fn ls(cfg: &Config) -> io::Result<()> {
    let path = Path::new(cfg.main.filepath.as_str());

    let text = fs::read_to_string(path)?;

    let /* mut */ lines_vec = text.lines().collect::<Vec<_>>();

    for (index, line) in lines_vec.into_iter().enumerate() {
        // println!("{}. {}", index + 1, line);
        println!("{}. {}", index + 1, line.parse_todo());
    }

    Ok(())
}

fn rm(cfg: &Config, line_nr: usize) -> io::Result<()> {
    let path = Path::new(cfg.main.filepath.as_str());

    ensure_parent(path)?;

    let mut lines: Vec<String> = fs::read_to_string(path)?
        .lines()
        .map(str::to_owned)
        .collect();

    let index = line_nr - 1;

    cfg.main
        .verbose
        .then(|| println!("rm({}): {}", line_nr, lines[index].parse_todo()));
    lines.remove(index);

    fs::write(path, lines.join("\n") + "\n")?;

    Ok(())
}

/// # TODO:
/// - Change commands to instead use `metatable`(**TODO**) from _argparse.rs_
pub fn interpret_args(cfg: &mut Config) -> io::Result<()> {
    let mut command_used = false;
    let args = env::args();
    for i in argparse::parse(args).unwrap() {
        match i {
            ParsedArgs::Command(cmd) => {
                match cmd {
                    Commands::Add(data) => {
                        add(cfg, data.as_str())?;
                    }
                    Commands::Done(nr) => {
                        done(cfg, nr)?;
                    }
                    Commands::Ls => {
                        ls(cfg)?;
                    }
                    Commands::Rm(nr) => {
                        rm(cfg, nr)?;
                    }
                }
                command_used = true;
            }
            ParsedArgs::Flag(flag) => match flag {
                Flags::File(file) => {
                    cfg.main.filepath = file;
                    // println!("Flag: Filing");
                }
                Flags::Help => {
                    println!("{HELP}");
                    exit(0)
                }
                Flags::Verbose => {
                    cfg.main.verbose = true;
                    cfg.main.verbose.then(|| println!("Flag: verbose=true"));
                }
                Flags::Version => {
                    let version = VERSION.map(|i| i.to_string()).join(".");
                    println!("v{version}");
                    exit(0)
                }
            },
        }
    }

    if !command_used {
        add_interactive(cfg)?;
    }

    Ok(())
}
