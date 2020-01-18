use crate::config::{BackendName, Config};
use crate::util::random_password;
use crate::{Backend, KeyringBackend, NitrokeyBackend, PwvltError};

#[derive(Default)]
/// The PasswordVault deals with managing multiple password backends.
pub struct PasswordVault {
    config: Config,
    stores: Vec<Box<dyn Backend>>,
}

impl PasswordVault {
    pub fn new(
        config: Config,
        nitrokey_unlock: Option<fn() -> Result<String, PwvltError>>,
    ) -> PasswordVault {
        let mut stores: Vec<Box<dyn Backend>> = Vec::with_capacity(2);
        for backend in &config.general.backends {
            match backend {
                BackendName::Nitrokey => {
                    let nitrokey_unlock = nitrokey_unlock
                        .expect("Must provide an unlock hook if you use the Nitrokey backend.");
                    match NitrokeyBackend::new(nitrokey_unlock) {
                        Ok(nk) => {
                            log::info!("Nitrokey backend loaded successfully!");
                            stores.push(Box::new(nk))
                        }
                        Err(e) => log::warn!("Failed to access Nitrokey: {}", e),
                    }
                }
                BackendName::Keyring => match KeyringBackend::new() {
                    Ok(kr) => {
                        log::info!("Keyring backend loaded successfully!");
                        stores.push(Box::new(kr))
                    }
                    Err(e) => log::warn!("Failed to access Keyring: {}", e),
                },
            }
        }
        PasswordVault { stores, config }
    }

    pub fn stores(&self) -> &Vec<Box<dyn Backend>> {
        &self.stores
    }

    pub fn password(&self, service: &str, username: &str) -> Result<String, PwvltError> {
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
        Err(PwvltError::PasswordNotFound)
    }

    pub fn set_password(
        &self,
        backend: usize,
        slot: usize,
        service: &str,
        username: &str,
        password: Option<&str>,
    ) -> Result<(), PwvltError> {
        let backend = &self.stores[backend];
        backend.set_password(
            slot,
            service,
            username,
            password.unwrap_or(&random_password(&self.config.password)?),
        )
    }

    pub fn default(&self, service: &str) -> Option<&String> {
        self.config.default.get(service)
    }
}
