use crate::error::PassStoreError;
use crate::pass_store::PassStore;

use nitrokey::{connect, CommandError, Device, DeviceWrapper, GetPasswordSafe, SLOT_COUNT};
use rpassword::prompt_password_stdout;

pub struct NitrokeyStore {
    device: DeviceWrapper,
}

impl NitrokeyStore {
    pub fn new() -> Result<NitrokeyStore, PassStoreError> {
        let device = connect()?;
        Ok(NitrokeyStore { device })
    }
}

impl PassStore for NitrokeyStore {
    fn password(&self, service: &str, username: &str) -> Result<String, PassStoreError> {
        let user_count = self.device.get_user_retry_count();
        if user_count < 1 {
            println!("Nitrokey must be unlocked using the admin pin!");
            println!("Please use the Nitrokey app to reset the user pin! Exiting.");
            return Err(PassStoreError::SkipError);
        };
        let pin =
            prompt_password_stdout(&format!("Nitrokey user pin ({} tries left):", user_count))?;
        let password_safe = self.device.get_password_safe(&pin)?;
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
        _service: &str,
        _username: &str,
        _password: &str,
    ) -> Result<(), PassStoreError> {
        unimplemented!("NitrokeyStore.set_password");
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
            err => unreachable!(
                "A NitrokeyKeyStore shouldn't generate a {} error.",
                err
            ),
        };
        println!("{}", message);
    }
}
