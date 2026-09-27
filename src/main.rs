use std::io;

fn main() {
    println!("`todo!(); rust?`");

    println!("Write a todo.txt entry, (or something...)");
    let mut entry = String::new();
    io::stdin()
        .read_line(&mut entry)
        .expect("Failed to read line");

    println!("todo.txt entry: {entry}");
}
