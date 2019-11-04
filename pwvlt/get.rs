use crate::error::{handle_nitrokey_error, PassStoreError};
use crate::keyring_store::KeyringStore;
use crate::nitrokey_store::NitrokeyStore;
use crate::pass_store::PassStore;

use clipboard::{ClipboardContext, ClipboardProvider};

use std::thread::sleep;
use std::time::Duration;

pub fn get_password(service: &str, username: &str) -> Result<(), PassStoreError> {
    // Search for the password on the Nitrokey.
    let nitro_store = NitrokeyStore::new()?;
    let password = match nitro_store.password(service, username) {
        Ok(pw) => {
            println!("Fetched password from Nitrokey.");
            pw
        }
        Err(e) => {
            handle_nitrokey_error(e);
            // Search the local keyring for the password.
            let keyring_store = KeyringStore::new();
            let pw = keyring_store.password(service, username)?;
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
