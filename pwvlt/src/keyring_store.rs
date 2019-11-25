use crate::error::PassStoreError;
use crate::pass_store::PassStore;

use keyring::Keyring;

#[derive(Default)]
pub struct KeyringStore {}

impl KeyringStore {
    pub fn new() -> KeyringStore {
        KeyringStore {}
    }
}

impl PassStore for KeyringStore {
    fn password(&self, service: &str, username: &str) -> Result<String, PassStoreError> {
        let keyring_entry = Keyring::new(service, username);
        keyring_entry.get_password().map_err(PassStoreError::from)
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

    fn handle_error(&self, err: PassStoreError) {
        let msg = match err {
            PassStoreError::KeyringError(err) => format!("{}", err),
            _ => unreachable!("A KeyringStore shouldn't generate a {} error.", err),
        };
        println!("{}", msg);
    }

    fn name(&self) -> &'static str {
        "Keyring"
    }
}
