use crate::{Backend, PwvltError, Slot};

use secret_service::{Collection, EncryptionType, SecretService};

use std::cell::RefCell;
use std::collections::HashMap;
use std::ptr::NonNull;

const NOT_SET: &str = "not_set";

pub struct KeyringBackend<'a> {
    secret_service: NonNull<SecretService>,
    collection: RefCell<Option<Collection<'a>>>,
    slots: RefCell<Option<Vec<Slot>>>,
}

impl<'a> Drop for KeyringBackend<'a> {
    fn drop(&mut self) {
        // make sure we release the secret_service field.
        unsafe { Box::from_raw(self.secret_service.as_ptr()) };
    }
}

impl<'a> KeyringBackend<'a> {
    pub fn new() -> Result<KeyringBackend<'a>, PwvltError> {
        let ss = Box::leak(Box::new(SecretService::new(EncryptionType::Dh)?));
        Ok(KeyringBackend {
            secret_service: NonNull::from(ss),
            collection: RefCell::new(None),
            slots: RefCell::new(None),
        })
    }

    /// Tries to unlock the collection. It also initialises collection and
    /// slots fields in case the underlying values are None.
    fn unlock_collection(&self) -> Result<(), PwvltError> {
        if self.collection.borrow().is_none() {
            let collection = unsafe {
                std::mem::transmute::<&SecretService, &'a SecretService>(
                    self.secret_service.as_ref(),
                )
            }
            .get_default_collection()?;
            let items = collection.get_all_items()?;
            let mut slots = Vec::with_capacity(items.len());
            for item in items {
                let mut attrs: HashMap<String, String> =
                    item.get_attributes()?.into_iter().collect();
                slots.push(Slot {
                    username: attrs.remove("username").unwrap_or_else(|| NOT_SET.into()),
                    service: attrs.remove("service").unwrap_or_else(|| NOT_SET.into()),
                });
            }
            self.collection.replace(Some(collection));
            self.slots.replace(Some(slots));
        }
        if let Some(collection) = &*self.collection.borrow() {
            if collection.is_locked()? {
                collection.unlock()?;
            }
        }
        Ok(())
    }

    /// Get slot `i`.
    fn slot(&self, i: usize) -> Option<Slot> {
        if let Some(slots) = &*self.slots.borrow() {
            slots.get(i).cloned()
        } else {
            panic!("Did you try to unlock_collection before using slot?");
        }
    }

    /// Removes slot `i` and pushes `slot` to the end of the `slots` vector.
    fn remove_and_add_slot(&self, i: usize, slot: Slot) {
        if let Some(slots) = &mut *self.slots.borrow_mut() {
            slots.remove(i);
            slots.push(slot);
        } else {
            panic!("Did you try to unlock_collection before using remove_and_add_slot?");
        }
    }

    fn delete_password(&self, service: &str, username: &str) -> Result<(), PwvltError> {
        if let Some(collection) = &*self.collection.borrow() {
            let attrs = vec![("service", service), ("username", username)];
            let search = collection.search_items(attrs)?;
            let item = search.get(0).ok_or(PwvltError::PasswordNotFound)?;
            item.delete()?;
            Ok(())
        } else {
            panic!("Did you try to unlock_collection before using delete_password?");
        }
    }
}

impl<'a> Backend for KeyringBackend<'a> {
    fn password(&self, service: &str, username: &str) -> Result<String, PwvltError> {
        self.unlock_collection()?;
        if let Some(collection) = &*self.collection.borrow() {
            let attrs = vec![("service", service), ("username", username)];
            let search = collection.search_items(attrs)?;
            let item = search.get(0).ok_or(PwvltError::PasswordNotFound)?;
            let secret_bytes = item.get_secret()?;
            Ok(String::from_utf8(secret_bytes)?)
        } else {
            unreachable!("Unlock collection should've errored.");
        }
    }

    fn set_password(
        &self,
        slot_index: usize,
        service: &str,
        username: &str,
        password: &str,
    ) -> Result<(), PwvltError> {
        self.unlock_collection()?;
        // if the slot_index is not out-of-bounds, then the user is trying to
        // replace this particular slot with new values.
        if let Some(slot) = self.slot(slot_index) {
            self.delete_password(&slot.service, &slot.username)?;
            self.remove_and_add_slot(slot_index, slot);
        };
        if let Some(collection) = &*self.collection.borrow() {
            let attrs = vec![
                ("service", service),
                ("username", username),
                ("application", "pwvlt"),
            ];
            let label = &format!("Password for {} on {}", username, service)[..];
            collection.create_item(
                label,
                attrs,
                password.as_bytes(),
                true, // replace
                "text/plain",
            )?;
            Ok(())
        } else {
            unreachable!("Unlock collection should've errored.");
        }
    }

    fn log_error(&self, err: PwvltError) {
        let msg = match err {
            PwvltError::Keyring(err) => err.to_string(),
            PwvltError::PasswordNotFound => "Password not found in Keyring".to_string(),
            _ => unreachable!("A KeyringBackend shouldn't generate a {} error.", err),
        };
        log::warn!("{}", msg);
    }

    fn name(&self) -> &'static str {
        "Keyring"
    }

    fn slots(&self) -> Result<Vec<Slot>, PwvltError> {
        self.unlock_collection()?;
        if let Some(slots) = &*self.slots.borrow() {
            let mut slots = slots.clone();
            slots.push(Default::default());
            Ok(slots)
        } else {
            unreachable!("Unlock collection should've errored.");
        }
    }
}
