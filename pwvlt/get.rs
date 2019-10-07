use crate::config::parse_config;
use crate::error::PassStoreError;
use crate::nitrokey::{get_password_from_nitrokey, handle_nitrokey_error};
use clap::Values;
use clipboard::{ClipboardContext, ClipboardProvider};
use keyring::Keyring;
use std::thread::sleep;
use std::time::Duration;

fn default_username(service: &str) -> Result<String, PassStoreError> {
    let toml = parse_config()?;
    match toml.get(service) {
        Some(username) => Ok(username.as_str().unwrap().into()),
        None => Err(PassStoreError::NoDefaultUser(service.into())),
    }
}

pub fn handle_get(mut values: Values) -> Result<(), PassStoreError> {
    let service = values.next().unwrap();
    let username = match values.next() {
        Some(username) => username.into(),
        None => default_username(service)?,
    };
    // Search for the password on the Nitrokey.
    let password = match get_password_from_nitrokey(service, &username) {
        Ok(pw) => {
            println!("Fetched password from Nitrokey.");
            pw
        },
        Err(e) => {
            handle_nitrokey_error(e);
            // search the local keyring for the password
            let keyring = Keyring::new(service, &username);
            let pw = keyring.get_password()?;
            println!("Fetched password from keyring.");
            pw
        }
    };
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(password)?;
    println!("Password copied to clipboard.");
    sleep(Duration::from_secs(5));
    ctx.set_contents(String::new())?;
    Ok(())
}
