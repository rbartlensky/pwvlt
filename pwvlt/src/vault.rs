use crate::config::Config;
use crate::error::PassStoreError;
use crate::keyring_store::KeyringStore;
use crate::nitrokey_store::NitrokeyStore;
use crate::pass_store::PassStore;
use crate::util::{looping_prompt, random_password};

use prettytable::{cell, row, Table};
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
                    match NitrokeyStore::new() {
                        Ok(nk) => stores.push(Box::new(nk)),
                        Err(e) => println!("Failed to access Nitrokey: {}", e),
                    }
                }
                "keyring" => stores.push(Box::new(KeyringStore::new())),
                b => println!("Skipping unknown backend '{}'", b),
            }
        }
        PasswordVault { stores, config }
    }

    pub fn password(&self, service: &str, username: &str) -> Result<String, PassStoreError> {
        for store in &self.stores {
            let res = store.password(service, username);
            if let Err(err) = res {
                store.handle_error(err);
            } else {
                return res;
            }
        }
        Err(PassStoreError::PasswordNotFound)
    }

    pub fn set_password(&self, service: &str, username: &str) -> Result<(), PassStoreError> {
        let backend = self.prompt_backend();
        let message = &format!(
            "New password for user {} (empty for randomly generated password):",
            username
        );
        let password = prompt_password_stdout(message)?;
        let password = if password.is_empty() {
            random_password(&self.config.password)?
        } else {
            password
        };
        self.stores[backend].set_password(service, username, &password)
    }

    fn prompt_backend(&self) -> usize {
        println!("Available password backends:");
        let mut table = Table::new();
        table.add_row(row!["#", "Backend"]);
        for (i, store) in self.stores.iter().enumerate() {
            table.add_row(row!(i.to_string(), store.name()));
        }
        table.printstd();
        looping_prompt("backend", self.stores.len() - 1)
    }

    pub fn default(&self, service: &str) -> Option<&String> {
        self.config.default.get(service)
    }
}
