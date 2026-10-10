mod argparse;
mod colors;
mod commands;
mod config;
mod live_input;
mod parser;

/// # TODO
/// - [?] Minimize number of `?` used
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cfg = config::load_config(String::new())?;
    cfg.main.verbose.then(|| println!("{cfg:#?}"));

    commands::interpret_args(&mut cfg)?;

    Ok(())
}
