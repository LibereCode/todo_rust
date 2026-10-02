/// # TODO
/// - [-] Have default values
/// - [ ] If config-file exist, merge it with defaults.
pub struct Config {
    pub filepath: String,
    pub verbose: bool,
}

impl Config {
    /// Returns a _struct_ of **default config**.
    pub fn new() -> Self {
        Self {
            filepath: String::from("todo.txt"),
            verbose: false,
        }
    }
}
