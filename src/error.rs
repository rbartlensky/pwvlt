use keyring::KeyringError;
use std::io::Error as IoError;
use std::error::Error;
use std::fmt;
use nitrokey::CommandError;

#[derive(Debug)]
pub enum PasswordStoreError {
    NoDefaultUser(String),
    PasswordNotFound,
    KeyringError(KeyringError),
    IoError(IoError),
    GeneralError(Box<dyn Error>),
    NitrokeyError(CommandError),
    SkipError,
    PasswordGenerationError(String),
}

impl fmt::Display for PasswordStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            PasswordStoreError::NoDefaultUser(service) => {
                format!("No default username set for {}", service)
            },
            PasswordStoreError::PasswordNotFound => {
                "Password not found.".to_string()
            },
            PasswordStoreError::KeyringError(err) => {
                format!("Keyring error: {}", err)
            },
            PasswordStoreError::IoError(err) => {
                format!("I/O error: {}", err)
            },
            PasswordStoreError::GeneralError(err) => {
                format!("Error: {}", err)
            },
            PasswordStoreError::NitrokeyError(err) => {
                format!("Nitrokey error: {}", err)
            },
            PasswordStoreError::SkipError => {
                "Skip error".to_string()
            },
            PasswordStoreError::PasswordGenerationError(err) => {
                format!("Error generating password: {}", err)
            }
        };
        write!(f, "{}", message)
    }
}

impl From<KeyringError> for PasswordStoreError {
    fn from(err: KeyringError) -> Self {
        Self::KeyringError(err)
    }
}

impl From<IoError> for PasswordStoreError {
    fn from(err: IoError) -> Self {
        Self::IoError(err)
    }
}

impl From<Box<dyn Error>> for PasswordStoreError {
    fn from(err: Box<dyn Error>) -> Self {
        Self::GeneralError(err)
    }
}

impl From<CommandError> for PasswordStoreError {
    fn from(err: CommandError) -> Self {
        Self::NitrokeyError(err)
    }
}
