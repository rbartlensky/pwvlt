use crate::error::PasswordStoreError;
use home::home_dir;
use std::fs::{read_to_string, write, DirBuilder, File};
use std::path::PathBuf;
use toml::{map::Map, Value};

const CONFIG_TOML: &str = "config.toml";

pub fn config_path() -> PathBuf {
    home_dir()
        .unwrap()
        .join(".config")
        .join("password-store-rust")
}

pub fn parse_config() -> Result<Map<String, Value>, PasswordStoreError> {
    let config_path = config_path();
    DirBuilder::new().recursive(true).create(&config_path)?;
    let defaults_file = config_path.join(CONFIG_TOML);
    if !defaults_file.exists() {
        File::create(defaults_file.clone())?;
    }
    let toml_payload = read_to_string(defaults_file)?;
    Ok(toml_payload
        .parse::<Value>()
        .ok()
        .and_then(|r| match r {
            Value::Table(table) => Some(table),
            _ => None,
        })
        .unwrap_or_else(Map::new))
}

pub fn write_toml(toml: Map<String, Value>) -> Result<(), PasswordStoreError> {
    write(
        config_path().join(CONFIG_TOML),
        format!("{}", Value::Table(toml)),
    )
    .map_err(PasswordStoreError::from)
}
