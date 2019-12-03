use serde::Deserialize;

/// Represents a user's configuration.
#[derive(Default, Deserialize)]
pub struct Config {
    pub general: General,
    pub password: Password,
}

#[derive(Default, Deserialize)]
pub struct General {
    pub backends: Vec<String>,
}

#[derive(Default, Deserialize)]
pub struct Password {
    pub length: usize,
    pub numbers: bool,
    pub lowercase_letters: bool,
    pub uppercase_letters: bool,
    pub symbols: bool,
    pub strict: bool,
}
