use crate::error::PassStoreError;
use crate::pass_store::{PassStore, Slot};

use nitrokey::{
    connect, CommandError, Device, DeviceWrapper, GetPasswordSafe, PasswordSafe, SLOT_COUNT,
};

use std::cell::RefCell;
use std::ptr::NonNull;

pub struct NitrokeyStore {
    device: NonNull<DeviceWrapper>,
    pws: RefCell<Option<PasswordSafe<'static>>>,
    unlock_hook: Box<dyn Fn() -> Result<String, PassStoreError>>,
}

impl Drop for NitrokeyStore {
    fn drop(&mut self) {
        let device = unsafe { Box::from_raw(self.device.as_ptr()) };
        if let Err(err) = device.lock() {
            eprintln!("Failed to lock the Nitrokey: {:?}", err);
        }
    }
}

impl NitrokeyStore {
    pub fn new(
        unlock_hook: Box<dyn Fn() -> Result<String, PassStoreError>>
    ) -> Result<NitrokeyStore, PassStoreError> {
        let device = Box::new(connect()?);
        let device = Box::leak(device);
        Ok(NitrokeyStore {
            device: NonNull::new(device).unwrap(),
            pws: RefCell::new(None),
            unlock_hook,
        })
    }

    fn device(&self) -> &'static DeviceWrapper {
        unsafe {
            std::mem::transmute(self.device.as_ref())
        }
    }

    pub fn unlock_safe(&self) -> Result<(), PassStoreError> {
        if self.pws.borrow().is_some() {
            return Ok(());
        }

        let user_count = self.device().get_user_retry_count();
        if user_count < 1 {
            log::error!("Nitrokey must be unlocked using the admin pin!");
            log::error!("Please use the Nitrokey app to reset the user pin! Exiting.");
            return Err(PassStoreError::SkipError);
        };
        let pin = (self.unlock_hook)()?;
        let pws = self.device()
            .get_password_safe(&pin)
            .map_err(PassStoreError::from)?;
        self.pws.replace(Some(pws));
        Ok(())
    }
}

impl PassStore for NitrokeyStore {
    fn password(&self, service: &str, username: &str) -> Result<String, PassStoreError> {
        self.unlock_safe()?;
        let pws_ref = &*self.pws.borrow();
        let pws = if let Some(pws) = pws_ref {
            pws
        } else {
            unreachable!("unlock_safe should've errored");
        };

        for slot in 0..SLOT_COUNT {
            if pws.get_slot_name(slot)? == service
                && pws.get_slot_login(slot)? == username
            {
                return pws
                    .get_slot_password(slot)
                    .map_err(PassStoreError::from);
            }
        }
        Err(PassStoreError::PasswordNotFound)
    }

    fn set_password(
        &self,
        slot: usize,
        service: &str,
        username: &str,
        password: &str,
    ) -> Result<(), PassStoreError> {
        self.unlock_safe()?;
        let pws_ref = &*self.pws.borrow();
        let pws = if let Some(pws) = pws_ref {
            pws
        } else {
            unreachable!("unlock_safe should've errored");
        };

        pws.write_slot(slot as u8, service, username, password)?;
        Ok(())
    }

    fn log_error(&self, err: PassStoreError) {
        let message = match err {
            PassStoreError::PasswordNotFound => "Password not found on Nitrokey!".into(),
            PassStoreError::SkipError => "Skipping Nitrokey search...".into(),
            PassStoreError::NitrokeyError(nke) => match nke {
                CommandError::Undefined => "Couldn't connect to the Nitrokey!".into(),
                CommandError::WrongPassword => "User pin was incorrect.".into(),
                err => format!("Nitrokey error: {}", err),
            },
            err => unreachable!("A NitrokeyKeyStore shouldn't generate a {} error.", err),
        };
        log::warn!("{}", message);
    }

    fn name(&self) -> &'static str {
        "Nitrokey"
    }

    fn slots(&self) -> Result<Vec<Slot>, PassStoreError> {
        self.unlock_safe()?;
        let pws_ref = &*self.pws.borrow();
        let pws = if let Some(pws) = pws_ref {
            pws
        } else {
            unreachable!("unlock_safe should've errored");
        };

        let slots = pws.get_slot_status()?
            .iter()
            .enumerate()
            .map(|(slot, programmed)| {
                if *programmed {
                    let service =
                        pws.get_slot_name(slot as u8).unwrap_or_else(|_| "".into());
                    let username =
                        pws.get_slot_login(slot as u8).unwrap_or_else(|_| "".into());
                    Slot { service, username }
                } else {
                    Default::default()
                }
            })
            .collect();
        Ok(slots)
    }
}
