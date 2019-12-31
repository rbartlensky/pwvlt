use crate::config::Config;
use crate::error::PassStoreError;
use crate::keyring_store::KeyringStore;
use crate::nitrokey_store::NitrokeyStore;
use crate::pass_store::PassStore;
use crate::util::{looping_prompt, random_password};

use rpassword::prompt_password_stdout;

#[derive(Default)]
pub struct PasswordVault {
    config: Config,
    stores: Vec<Box<dyn PassStore>>,
}

impl PasswordVault {
    pub fn new(config: Config) -> PasswordVault {
        let mut stores: Vec<Box<dyn PassStore>> = Vec::with_capacity(2);
        for backend in &config.general.backends {
            match backend.as_str() {
                "nitrokey" => {
                    let unlock_hook = || -> Result<String, PassStoreError> {
                        let pin =
                            prompt_password_stdout("Nitrokey user pin:")?;
                        Ok(pin)

                    };
                    match NitrokeyStore::new(Box::new(unlock_hook)) {
                        Ok(nk) => {
                            log::info!("Nitrokey backend loaded successfully!");
                            stores.push(Box::new(nk))
                        }
                        Err(e) => log::warn!("Failed to access Nitrokey: {}", e),
                    }
                }
                "keyring" => {
                    log::info!("Keyring backend loaded successfully!");
                    stores.push(Box::new(KeyringStore::new()))
                }
                b => log::warn!("Skipping unknown backend '{}'", b),
            }
        }
        PasswordVault { stores, config }
    }

    pub fn stores(&self) -> &Vec<Box<dyn PassStore>> {
        &self.stores
    }

    pub fn password(&self, service: &str, username: &str) -> Result<String, PassStoreError> {
        for store in &self.stores {
            let res = store.password(service, username);
            log::info!("Looking for password in {}.", store.name());
            if let Err(err) = res {
                store.log_error(err);
            } else {
                log::info!("Found password in {}.", store.name());
                return res;
            }
        }
        Err(PassStoreError::PasswordNotFound)
    }

    pub fn set_password(&self, backend: usize, service: &str, username: &str) -> Result<(), PassStoreError> {
        let message = &format!(
            "New password for user {} (empty for randomly generated password):",
            username
        );
        log::info!("Prompting for new password.");
        let password = prompt_password_stdout(message)?;
        let password = if password.is_empty() {
            random_password(&self.config.password)?
        } else {
            password
        };

        self.stores[backend].set_password(service, username, &password)
    }

    pub fn default(&self, service: &str) -> Option<&String> {
        self.config.default.get(service)
    }
}
