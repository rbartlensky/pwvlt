use crate::error::PasswordStoreError;
use nitrokey::{connect, CommandError, Device, GetPasswordSafe, SLOT_COUNT};
use rpassword::prompt_password_stdout;

pub fn get_password_from_nitrokey<'a>(
    service: &'a str,
    username: &'a str,
) -> Result<String, PasswordStoreError> {
    let device = connect()?;
    let user_count = device.get_user_retry_count();
    if user_count < 1 {
        println!(
            "Nitrokey must be unlocked with admin pin!
Please use the nitrokey app to reset the user pin!"
        );
        return Err(PasswordStoreError::SkipError);
    };
    let pin = prompt_password_stdout(&format!("Nitrokey user pin ({} tries left):", user_count))?;
    let password_safe = device.get_password_safe(&pin)?;
    for slot in 0..SLOT_COUNT {
        if password_safe.get_slot_name(slot)? == service
            && password_safe.get_slot_login(slot)? == username
        {
            return password_safe
                .get_slot_password(slot)
                .map_err(|e| PasswordStoreError::from(e));
        }
    }
    Err(PasswordStoreError::PasswordNotFound)
}

pub fn handle_nitrokey_error(err: PasswordStoreError) {
    let message = match err {
        PasswordStoreError::PasswordNotFound => "Password not found on the Nitrokey!".into(),
        PasswordStoreError::SkipError => "Skipping Nitrokey search...".into(),
        PasswordStoreError::NitrokeyError(nke) => match nke {
            CommandError::Undefined => {
                "Couldn't connect to the Nitrokey! Skipping Nitrokey search...".into()
            }
            CommandError::WrongPassword => {
                "User pin was incorrect. Skipping Nitrokey search...".into()
            }
            err => format!("Nitrokey error: {}", err),
        },
        err => unreachable!(
            "Call to get_password_from_nitrokey shouldn't generate a {:?} error",
            err
        ),
    };
    println!("{}", message);
}
