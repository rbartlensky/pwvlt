use crate::config;
use crate::error::PwvltError;

use passwords::PasswordGenerator;

use std::fmt::Display;
use std::io::{self, stdout, BufRead, Write};
use std::ops::Sub;
use std::str::FromStr;

pub fn random_password(config: &config::Password) -> Result<String, PwvltError> {
    let pg = PasswordGenerator {
        length: config.length,
        numbers: config.numbers,
        lowercase_letters: config.lowercase_letters,
        uppercase_letters: config.uppercase_letters,
        symbols: config.symbols,
        strict: config.strict,
    };
    log::info!("Generating random password.");
    pg.generate_one()
        .map_err(|e| PwvltError::PasswordGeneration(e.into()))
}

pub fn looping_prompt<T>(item: &str, max_val: T) -> T
where
    T: Ord + Sub + Display + FromStr,
{
    loop {
        let item_val = prompt_string(format!("Select {} (0-{})", item, max_val));
        if let Ok(item_val) = item_val.parse::<T>() {
            if item_val <= max_val {
                return item_val;
            }
            println!("Invalid {} number: {}", item, item_val);
        } else {
            println!("Invalid {}: {}", item, item_val);
        }
    }
}

pub fn prompt_string<S: AsRef<str>>(message: S) -> String {
    print!("{}: ", message.as_ref());
    stdout().flush().unwrap();
    let mut item_val = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut item_val).unwrap();
    item_val.trim().into()
}
