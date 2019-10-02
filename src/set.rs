use crate::config::{parse_config, write_toml};
use crate::error::PasswordStoreError;
use clap::Values;
use keyring::Keyring;
use passwords::PasswordGenerator;
use toml::Value;

fn random_password() -> Result<String, PasswordStoreError> {
    let pg = PasswordGenerator {
        length: 22,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        strict: true,
    };
    pg.generate_one()
        .map_err(|e| PasswordStoreError::PasswordGenerationError(e.into()))
}

pub fn handle_set(mut values: Values) -> Result<(), PasswordStoreError> {
    let service = values.next().unwrap();
    let username = values.next().unwrap();
    let password = match values.next() {
        Some(password) => password.into(),
        None => random_password()?,
    };
    let keyring = Keyring::new(service, username);
    keyring.set_password(&password)?;
    let mut toml = parse_config()?;
    if toml.get(service).is_none() {
        toml.insert(service.into(), Value::String(username.into()));
    }
    write_toml(toml)
}
