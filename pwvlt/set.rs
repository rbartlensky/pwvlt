use crate::config::{parse_config, write_toml};
use crate::error::PassStoreError;
use crate::util::random_password;
use clap::Values;
use keyring::Keyring;
use toml::Value;

pub fn handle_set(mut values: Values) -> Result<(), PassStoreError> {
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
