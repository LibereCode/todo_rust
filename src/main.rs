use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufRead, Write},
    path::Path,
};

// /// > [!TODO]
// /// > Use this in some `print_help()` function
// const HELP: &str = "\
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
// ";

const TODO_PATH: &str = "out.todo.txt";

/// > [!TODO] Minimize number of `?` used
fn ensure_parent(path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

/// > [!TODO] Minimize number of `?` used
fn try_write(path: &Path, data: &str) -> io::Result<()> {
    ensure_parent(path)?;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    write!(file, "{data}")?;

    Ok(())
}

/// > [!TODO] Minimize number of `?` used
fn main() -> io::Result<()> {
    println!("`todo!(); rust?`");

    println!("Write a todo.txt entry, (or something...)");
    let mut entry = String::new();
    io::stdin()
        .read_line(&mut entry)
        .expect("Failed to read line");

    print!("todo.txt entry: {entry}");

    let path = Path::new(TODO_PATH);

    try_write(path, entry.as_str())?;

    let cunt = fs::read_to_string(path).unwrap_or_default();
    for (index, line) in cunt.lines().enumerate() {
        println!("{}: {}", index + 1, line)
    }

    Ok(())
}
