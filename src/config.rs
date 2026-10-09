use serde::Deserialize;
use std::{env, fs};

/// # TODO
/// - [x] Have default values
/// - [ ] If config-file exist, merge it with defaults.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub main: MainConfig,
}

#[derive(Debug, Deserialize)]
pub struct MainConfig {
    pub filepath: String,
    pub verbose: bool,
}

// impl Config {
//     /// Returns a _struct_ of **default config**.
//     pub fn new() -> Self {
//         Self {
//             main: MainConfig {
//                 filepath: String::from("todo.txt"),
//                 verbose: false,
//             },
//         }
//     }
//
//     // /// WARN: Doesn't work yet
//     // /// - `hashmap.get(key)` just returns `None`
//     // fn from_hashmap(hashmap: &HashMap<&str, &str>) -> Self {
//     //     let cfg = Config::new();
//     //     let filepath = hashmap
//     //         .get("filepath")
//     //         .map(|&value| value.trim())
//     //         .filter(|&value| !value.is_empty())
//     //         .unwrap_or(cfg.filepath.as_str())
//     //         .to_string();
//     //
//     //     let verbose = hashmap
//     //         .get("verbose")
//     //         .and_then(|&value| value.trim().parse::<bool>().ok())
//     //         .unwrap_or(cfg.verbose);
//     //
//     //     // println!(
//     //     //     "{filepath} ; {verbose} ; {:?} ; {:?}",
//     //     //     hashmap.get("filepath"),
//     //     //     hashmap.get("verbose")
//     //     // );
//     //     Config { filepath, verbose }
//     // }
// }

/// # TODO
/// - [ ] Should instead return a `Config`
/// - [ ] Parse the _String_ (text from a `config.ini` file)
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
