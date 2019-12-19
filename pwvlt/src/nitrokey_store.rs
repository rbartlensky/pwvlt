use crate::error::PassStoreError;
use crate::pass_store::PassStore;
use crate::util::looping_prompt;

use nitrokey::{
    connect, CommandError, Device, DeviceWrapper, GetPasswordSafe, PasswordSafe, SLOT_COUNT,
};
use prettytable::{cell, row, Table};
use rpassword::prompt_password_stdout;

use std::io::{stdout, Write};

pub struct NitrokeyStore {
    device: DeviceWrapper,
}

impl Drop for NitrokeyStore {
    fn drop(&mut self) {
        if let Err(err) = self.device.lock() {
            eprintln!("Failed to lock the Nitrokey: {:?}", err);
        }
    }
}

impl NitrokeyStore {
    pub fn new() -> Result<NitrokeyStore, PassStoreError> {
        let device = connect()?;
        Ok(NitrokeyStore { device })
    }

    pub fn unlock_safe(&self) -> Result<PasswordSafe, PassStoreError> {
        let user_count = self.device.get_user_retry_count();
        if user_count < 1 {
            println!("Nitrokey must be unlocked using the admin pin!");
            println!("Please use the Nitrokey app to reset the user pin! Exiting.");
            return Err(PassStoreError::SkipError);
        };
        let pin =
            prompt_password_stdout(&format!("Nitrokey user pin ({} tries left):", user_count))?;
        self.device
            .get_password_safe(&pin)
            .map_err(PassStoreError::from)
    }

    fn print_slots(&self, pws: &PasswordSafe) -> Result<(), PassStoreError> {
        print!("Retrieving the Nitrokey slots...\r");
        stdout().flush().unwrap();
        let mut table = Table::new();
        table.add_row(row!["Slot", "Service", "Username"]);
        pws.get_slot_status()?
            .iter()
            .enumerate()
            .for_each(|(slot, programmed)| {
                let (name, login) = if *programmed {
                    let name = pws.get_slot_name(slot as u8).unwrap_or_else(|_| "".into());
                    let login = pws.get_slot_login(slot as u8).unwrap_or_else(|_| "".into());
                    (name, login)
                } else {
                    ("".into(), "".into())
                };
                table.add_row(row![slot.to_string(), name, login]);
            });
        table.printstd();
        Ok(())
    }
}

impl PassStore for NitrokeyStore {
    fn password(&self, service: &str, username: &str) -> Result<String, PassStoreError> {
        let password_safe = self.unlock_safe()?;
        for slot in 0..SLOT_COUNT {
            if password_safe.get_slot_name(slot)? == service
                && password_safe.get_slot_login(slot)? == username
            {
                return password_safe
                    .get_slot_password(slot)
                    .map_err(PassStoreError::from);
            }
        }
        Err(PassStoreError::PasswordNotFound)
    }

    fn set_password(
        &self,
        service: &str,
        username: &str,
        password: &str,
    ) -> Result<(), PassStoreError> {
        let password_safe = self.unlock_safe()?;
        self.print_slots(&password_safe)?;
        let slot = looping_prompt("slot", SLOT_COUNT - 1);
        password_safe.write_slot(slot, service, username, password)?;
        Ok(())
    }

    fn handle_error(&self, err: PassStoreError) {
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
}
