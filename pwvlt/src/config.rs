use serde::{Deserialize, Serialize};

use std::collections::HashMap;

/// Represents a user's configuration.
#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    pub general: General,
    pub password: Password,
    #[serde(default)]
    /// A mapping from services to usernames. Users can set default
    /// usernames for specific services.
    pub default: HashMap<String, String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BackendName {
    Nitrokey,
    Keyring,
}

#[derive(Default, Deserialize, Serialize)]
pub struct General {
    /// The backends that pwvlt will load.
    pub backends: Vec<BackendName>,
}

#[derive(Default, Deserialize, Serialize)]
/// The fields of this struct are already described here:
/// https://docs.rs/passwords/1.1.5/passwords/struct.PasswordGenerator.html
pub struct Password {
    pub length: usize,
    pub numbers: bool,
    pub lowercase_letters: bool,
    pub uppercase_letters: bool,
    pub symbols: bool,
    pub strict: bool,
}
