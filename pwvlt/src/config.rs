use serde::{Deserialize, Serialize};

use std::collections::HashMap;

/// Represents a user's configuration.
#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    pub general: General,
    pub password: Password,
    #[serde(default)]
    pub default: HashMap<String, String>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct General {
    pub backends: Vec<String>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Password {
    pub length: usize,
    pub numbers: bool,
    pub lowercase_letters: bool,
    pub uppercase_letters: bool,
    pub symbols: bool,
    pub strict: bool,
}
