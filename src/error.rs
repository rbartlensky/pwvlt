use keyring::KeyringError;
use std::io::Error as IOError;
use std::error::Error;
use nitrokey::CommandError;

#[derive(Debug)]
pub enum PasswordStoreError {
    NoDefaultUser(String),
    PasswordNotFound,
    KeyringError(KeyringError),
    IOError(IOError),
    GeneralError(Box<dyn Error>),
    NitrokeyError(CommandError),
    SkipError,
    PasswordGenerationError(String),
}

impl From<KeyringError> for PasswordStoreError {
    fn from(err: KeyringError) -> Self {
        Self::KeyringError(err)
    }
}

impl From<IOError> for PasswordStoreError {
    fn from(err: IOError) -> Self {
        Self::IOError(err)
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
