use keyring::KeyringError;
use std::io::Error as IoError;
use std::error::Error;
use std::fmt;
use nitrokey::CommandError;

#[derive(Debug)]
pub enum PassStoreError {
    NoDefaultUser(String),
    PasswordNotFound,
    KeyringError(KeyringError),
    IoError(IoError),
    GeneralError(Box<dyn Error>),
    NitrokeyError(CommandError),
    SkipError,
    PasswordGenerationError(String),
}

impl fmt::Display for PassStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            PassStoreError::NoDefaultUser(service) => {
                format!("No default username set for {}", service)
            },
            PassStoreError::PasswordNotFound => {
                "Password not found.".to_string()
            },
            PassStoreError::KeyringError(err) => {
                format!("Keyring error: {}", err)
            },
            PassStoreError::IoError(err) => {
                format!("I/O error: {}", err)
            },
            PassStoreError::GeneralError(err) => {
                format!("Error: {}", err)
            },
            PassStoreError::NitrokeyError(err) => {
                format!("Nitrokey error: {}", err)
            },
            PassStoreError::SkipError => {
                "Skip error".to_string()
            },
            PassStoreError::PasswordGenerationError(err) => {
                format!("Error generating password: {}", err)
            }
        };
        write!(f, "{}", message)
    }
}

impl From<KeyringError> for PassStoreError {
    fn from(err: KeyringError) -> Self {
        Self::KeyringError(err)
    }
}

impl From<IoError> for PassStoreError {
    fn from(err: IoError) -> Self {
        Self::IoError(err)
    }
}

impl From<Box<dyn Error>> for PassStoreError {
    fn from(err: Box<dyn Error>) -> Self {
        Self::GeneralError(err)
    }
}

impl From<CommandError> for PassStoreError {
    fn from(err: CommandError) -> Self {
        Self::NitrokeyError(err)
    }
}
