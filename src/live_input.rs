use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal,
};

use crate::parser::{ParseMetaTable, ParseStr};

/// Will print the current input (ie: _update_) on each key press.
/// Parse / colorize directly here for those prints.
///
/// Returns the final String on **Return**-key.
///
/// # Old (simple) version
/// ```rust
/// /* snip */
/// let mut entry = String::new();
/// io::stdin()
///     .read_line(&mut entry)
///     .expect("Failed to read line");
/// /* snip */
/// ```
pub fn read_input_live() -> io::Result<String> {
    terminal::enable_raw_mode()?;

    let result = read_input();

    // Always restore normal terminal behavior.
    terminal::disable_raw_mode()?;

    result
}

/// # TODO
/// - [ ] Automatically show/add current date after **(P)rio** (or prepend if prio's missing)
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
                }

                KeyCode::Backspace => {
                    input.pop();
                }

                KeyCode::Enter => {
                    println!("\r");
                    io::stdout().flush()?;

                    return Ok(input);
                }

                KeyCode::Esc => {
                    input.clear();
                }

                _ => {}
            }

            let input_parsed = input.as_str().parse2metatable().parse_todo();
            print!(
                "\r\x1b[2K  parsed: {input_parsed}; {:>30}",
                format!("raw: {input}")
            );
            std::io::stdout().flush()?;
        }
    }
}
