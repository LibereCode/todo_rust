use serde::Deserialize;
use std::{env, fs};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub main: MainConfig,
}

#[derive(Debug, Deserialize)]
pub struct MainConfig {
    pub filepath: String,
    pub verbose: bool,
}

pub fn load_config(filepath: String) -> Result<Config, Box<dyn std::error::Error>> {
    let path = if !filepath.is_empty() {
        filepath
    } else if env::var("TODO_RUST_CONFIG").is_ok() {
        env::var("TODO_RUST_CONFIG").unwrap()
    } else {
        let home = env::var("HOME").expect("$HOME variable is not set.");
        home + "/.config/todo_rust/config.toml"
    };

    let text = fs::read_to_string(path)?;
    let config = toml::from_str(&text)?;

    Ok(config)
}
