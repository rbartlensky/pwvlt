use nitrokey::CommandError;
use secret_service::SsError;

use std::fmt;

#[derive(derive_more::From, Debug)]
pub enum PwvltError {
    /// Usually returned by the `password` method.
    PasswordNotFound,
    Keyring(SsError),
    Io(std::io::Error),
    Nitrokey(CommandError),
    /// A skip error is returned by `NitrokeyStore::new`, when the admin pin
    /// is needed to unlock the Nitrokey.
    Skip,
    PasswordGeneration(String),
    Utf8(std::string::FromUtf8Error),
}

impl fmt::Display for PwvltError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            PwvltError::Utf8(err) => format!("{}", err),
            PwvltError::PasswordNotFound => "Password not found.".to_string(),
            PwvltError::Keyring(err) => format!("Keyring error: {}", err),
            PwvltError::Io(err) => format!("I/O error: {}", err),
            PwvltError::Nitrokey(err) => format!("Nitrokey error: {}", err),
            PwvltError::Skip => "Skip error".to_string(),
            PwvltError::PasswordGeneration(err) => format!("Error generating password: {}", err),
        };
        write!(f, "{}", message)
    }
}
