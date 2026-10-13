use std::io::{self, Write};

use chrono::Local;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal,
};

use crate::parser::{BodyPart, ParseMetaTable, ParseStr, Priority};

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

fn read_input() -> io::Result<String> {
    let mut input = String::new();
    let mut meta = input.parse2metatable();

    println!("Live Input Preview\r");
    print!("Parsed:\r\n");
    print!("Raw:\r\n\x1b[1A");
    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            // exit on Ctrl + c
            if key.modifiers.contains(event::KeyModifiers::CONTROL)
                && key.code == KeyCode::Char('c')
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

                    return Ok(meta.to_raw());
                }

                KeyCode::Esc => {
                    input.clear();
                }

                _ => {}
            }

            meta = input.parse2metatable();

            if meta.date.is_none() {
                let naive_date = Local::now().date_naive();
                let index = match meta.prio {
                    Priority::None => 0,
                    _ => 1,
                };
                meta.body.insert(index, BodyPart::Date(naive_date));
                meta.date = Some(naive_date);
            }

            let input_parsed = meta.colorize();

            // print!(
            //     "\r\x1b[2K  parsed: {input_parsed}; {:>30}",
            //     format!("raw: {input}")
            // );
            print!(
                "\r\x1b[2K\x1b[1A\x1b[2KParsed:\t{input_parsed}\r\n{:#}",
                format!("Raw:\t{input}")
            );

            std::io::stdout().flush()?;
        }
    }
}
