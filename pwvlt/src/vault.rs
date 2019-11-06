use crate::error::PassStoreError;
use crate::keyring_store::KeyringStore;
use crate::nitrokey_store::NitrokeyStore;
use crate::pass_store::PassStore;

macro_rules! push_store {
    ($v: expr, $store: expr, $name: tt) => {
        match $store {
            Some(ref store) => $v.push(store),
            None => println!("Skipping {}", $name),
        }
    };
}

pub struct PasswordVault {
    nitrokey_store: Option<NitrokeyStore>,
    keyring_store: Option<KeyringStore>,
}

impl PasswordVault {
    pub fn new() -> PasswordVault {
        let nk = NitrokeyStore::new().ok();
        PasswordVault {
            nitrokey_store: nk,
            keyring_store: Some(KeyringStore::new()),
        }
    }

    pub fn stores<'a>(&'a self) -> Vec<&'a dyn PassStore> {
        let mut v: Vec<&dyn PassStore> = Vec::with_capacity(2);
        push_store!(v, self.nitrokey_store, "Nitrokey");
        push_store!(v, self.keyring_store, "Keyring");
        assert!(
            !v.is_empty(),
            "No active stores to query... Skipping password search!"
        );
        v
    }

    pub fn password(&self, service: &str, username: &str) -> Result<String, PassStoreError> {
        for store in self.stores() {
            let res = store.password(service, username);
            if let Err(err) = res {
                store.handle_error(err);
            } else {
                return res;
            }
        }
        Err(PassStoreError::PasswordNotFound)
    }
}
