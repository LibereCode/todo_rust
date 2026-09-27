use std::{fs, io};

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

fn main() -> io::Result<()> {
    println!("`todo!(); rust?`");

    println!("Write a todo.txt entry, (or something...)");
    let mut entry = String::new();
    io::stdin()
        .read_line(&mut entry)
        .expect("Failed to read line");

    println!("todo.txt entry: {entry}");

    let file = "out.todo.txt";
    fs::write(file, String::from(entry))?;

    println!("Wrote entry to file: ./{file}");

    Ok(())
}
