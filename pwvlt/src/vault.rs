use crate::error::PassStoreError;
use crate::keyring_store::KeyringStore;
use crate::nitrokey_store::NitrokeyStore;
use crate::pass_store::PassStore;
use crate::util::{looping_prompt, random_password};

use prettytable::{cell, row, Table};
use rpassword::prompt_password_stdout;

#[derive(Default)]
pub struct PasswordVault {
    stores: Vec<Box<dyn PassStore>>,
}

impl PasswordVault {
    pub fn new() -> PasswordVault {
        let mut stores: Vec<Box<dyn PassStore>> = Vec::with_capacity(2);
        if let Ok(nk) = NitrokeyStore::new() {
            stores.push(Box::new(nk));
        }
        stores.push(Box::new(KeyringStore::new()));
        PasswordVault { stores }
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
            random_password()?
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
}
