use crate::error::PassStoreError;
use crate::keyring_store::KeyringStore;
use crate::pass_store::PassStore;
use crate::util::random_password;

use rpassword::prompt_password_stdout;

// XXX This only ever updates the keyring.
pub fn set_password(service: &str, username: &str) -> Result<(), PassStoreError> {
    let keyring_store = KeyringStore::new();
    let message = &format!(
        "New password for user {} (empty for randomly generated password):",
        username
    );
    let password = prompt_password_stdout(message)?;
    let password = if password.is_empty() {
        random_password()?
    } else {
        password
    };
    keyring_store.set_password(service, username, &password)?;
    Ok(())
}
