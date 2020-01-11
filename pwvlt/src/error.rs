use secret_service::SsError;
use nitrokey::CommandError;

use std::error::Error;
use std::fmt;
use std::io::Error as IoError;

#[derive(derive_more::From, Debug)]
pub enum PassStoreError {
    PasswordNotFound,
    Keyring(SsError),
    IoError(IoError),
    GeneralError(Box<dyn Error>),
    NitrokeyError(CommandError),
    SkipError,
    PasswordGenerationError(String),
    LockedError,
    Utf8(std::string::FromUtf8Error),
}

impl fmt::Display for PassStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            PassStoreError::Utf8(err) => format!("{}", err),
            PassStoreError::PasswordNotFound => "Password not found.".to_string(),
            PassStoreError::Keyring(err) => format!("Keyring error: {}", err),
            PassStoreError::IoError(err) => format!("I/O error: {}", err),
            PassStoreError::GeneralError(err) => format!("Error: {}", err),
            PassStoreError::NitrokeyError(err) => format!("Nitrokey error: {}", err),
            PassStoreError::SkipError => "Skip error".to_string(),
            PassStoreError::PasswordGenerationError(err) => {
                format!("Error generating password: {}", err)
            },
            PassStoreError::LockedError => "Backend still locked".to_string(),
        };
        write!(f, "{}", message)
    }
}
