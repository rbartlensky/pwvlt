use crate::{config, PwvltError};

use passwords::PasswordGenerator;

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
