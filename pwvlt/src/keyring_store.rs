use crate::error::PassStoreError;
use crate::pass_store::PassStore;

use keyring::Keyring;

pub struct KeyringStore {}

impl KeyringStore {
    pub fn new() -> KeyringStore {
        KeyringStore {}
    }
}

impl PassStore for KeyringStore {
    fn password(&self, service: &str, username: &str) -> Result<String, PassStoreError> {
        let keyring_entry = Keyring::new(service, username);
        // search the local keyring for the password
        let pw = keyring_entry.get_password()?;
        println!("Fetched password from keyring.");
        Ok(pw)
    }

    fn set_password(
        &self,
        service: &str,
        username: &str,
        password: &str,
    ) -> Result<(), PassStoreError> {
        Keyring::new(service, username)
            .set_password(&password)
            .map_err(PassStoreError::from)
    }
}
