use std::{
    fs::{self},
    io::{self, Write},
    path::Path,
};

/// # TODO
/// move this to `config()`
const TODO_PATH: &str = "out.todo.txt";

// /// # TODO
// /// - [ ] Use this in some `print_help()` function
// const HELP: &str = "
// todo_rust — read/write todo.txt. rust btw.
//
// Usage:
//     todo_rust [OPTIONS]
//
// Options:
//     -f, --file <FILE>    Target file
//     -v, --verbose        Be more verbose
//     -h, --help           Print this help
//     -V, --version        Print version
//
// Commands:
//     add [ENTRY]          Add an entry. Leave empty to get an input prompt.
//     done [INDEX]         Mark an entry as done by index. Leave empty to to get an input prompt.
//     ls                   List added entries (read todo file).
//     rm [INDEX]           Remove an entry by index. Leave empty to to get an input prompt.
// ";

// /// # TODO
// /// - [ ] Read env-var or config file
// /// - [ ] Default values
// fn config() {
// }

// /// # TODO
// /// - [ ] Implement `argparse`: reading --flags and commands (see HELP)
// fn argparse() {
// }

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

trait Colorize {
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
/// - [ ] Minimize number of `?` used
fn main() -> io::Result<()> {
    println!(
        "{}",
        "Write a todo.txt entry, (or something...)".color("yellow")
    );
    let mut entry = String::new();
    io::stdin()
        .read_line(&mut entry)
        .expect("Failed to read line");
    println!("[DEBUG] todo.txt entry: \"{entry}\"");

    let path = Path::new(TODO_PATH);

    try_write(path, entry.as_str())?;

    let text = fs::read_to_string(path).unwrap_or_default();
    println!("DEBUG: text = \"{text}\"");

    // TODO use something like this for the parser
    let /* mut */ lines_vec = text.lines().collect::<Vec<_>>();

    for (index, line) in lines_vec.into_iter().enumerate() {
        println!("{}. {}", index + 1, line);
    }

    Ok(())
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
