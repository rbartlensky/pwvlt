use crate::error::PwvltError;
use crate::pass_store::{PassStore, Slot};

use nitrokey::{
    connect, CommandError, Device, DeviceWrapper, GetPasswordSafe, PasswordSafe, SLOT_COUNT,
};

use std::cell::RefCell;
use std::ptr::NonNull;

pub struct NitrokeyStore<'a> {
    device: NonNull<DeviceWrapper>,
    pws: RefCell<Option<PasswordSafe<'a>>>,
    unlock_hook: Box<dyn Fn() -> Result<String, PwvltError>>,
}

impl<'a> Drop for NitrokeyStore<'a> {
    fn drop(&mut self) {
        let device = unsafe { Box::from_raw(self.device.as_ptr()) };
        if let Err(err) = device.lock() {
            eprintln!("Failed to lock the Nitrokey: {:?}", err);
        }
    }
}

impl<'a> NitrokeyStore<'a> {
    pub fn new(
        unlock_hook: Box<dyn Fn() -> Result<String, PwvltError>>
    ) -> Result<NitrokeyStore<'a>, PwvltError> {
        let device = Box::new(connect()?);
        let device = Box::leak(device);
        Ok(NitrokeyStore {
            device: NonNull::new(device).unwrap(),
            pws: RefCell::new(None),
            unlock_hook,
        })
    }

    fn device(&self) -> &'a DeviceWrapper {
        unsafe {
            std::mem::transmute(self.device.as_ref())
        }
    }

    pub fn unlock_safe(&self) -> Result<(), PwvltError> {
        if self.pws.borrow().is_some() {
            return Ok(());
        }

        let user_count = self.device().get_user_retry_count();
        if user_count < 1 {
            log::error!("Nitrokey must be unlocked using the admin pin!");
            log::error!("Please use the Nitrokey app to reset the user pin! Exiting.");
            return Err(PwvltError::Skip);
        };
        let pin = (self.unlock_hook)()?;
        let pws = self.device()
            .get_password_safe(&pin)
            .map_err(PwvltError::from)?;
        self.pws.replace(Some(pws));
        Ok(())
    }
}

impl<'a> PassStore for NitrokeyStore<'a> {
    fn password(&self, service: &str, username: &str) -> Result<String, PwvltError> {
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
                    .map_err(PwvltError::from);
            }
        }
        Err(PwvltError::PasswordNotFound)
    }

    fn set_password(
        &self,
        slot: usize,
        service: &str,
        username: &str,
        password: &str,
    ) -> Result<(), PwvltError> {
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

    fn log_error(&self, err: PwvltError) {
        let message = match err {
            PwvltError::PasswordNotFound => "Password not found on Nitrokey!".into(),
            PwvltError::Skip => "Skipping Nitrokey search...".into(),
            PwvltError::Nitrokey(nke) => match nke {
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

    fn slots(&self) -> Result<Vec<Slot>, PwvltError> {
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
