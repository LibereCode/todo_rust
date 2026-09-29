use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal,
};

pub fn read_input_live() -> io::Result<String> {
    terminal::enable_raw_mode()?;

    let result = read_input();

    // Always restore normal terminal behavior.
    terminal::disable_raw_mode()?;

    result
}

/// # TODO
/// - [ ] Colorize live
/// - [ ] Cancel on CTRL-c
fn read_input() -> io::Result<String> {
    let mut input = String::new();

    println!("Preview:\r");
    loop {
        if let Event::Key(key) = event::read()? {
            // On some platforms, key events can include press/release events.
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if key.code == KeyCode::Char('c')
                && key.modifiers.contains(event::KeyModifiers::CONTROL)
            {
                print!("\r\n");
                io::stdout().flush()?;

                return Err(io::Error::new(
                    io::ErrorKind::Interrupted,
                    "input cancelled",
                ));
            }

            match key.code {
                KeyCode::Char(character) => {
                    input.push(character);

                    // This is the current string after the key press.
                    // print!("Current input: {input}\r");

                    // Or process it here:
                    // process_input(&input);
                }

                KeyCode::Backspace => {
                    input.pop();

                    // println!("Current input: {input}\r");
                }

                KeyCode::Enter => {
                    println!("\r");
                    io::stdout().flush()?;

                    return Ok(input);
                }

                KeyCode::Esc => {
                    input.clear();
                    // println!("\nInput cleared");
                }

                _ => {}
            }

            print!("\r\x1b[2K{input}");
            std::io::stdout().flush()?;
        }
    }
}
