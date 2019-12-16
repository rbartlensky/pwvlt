use pwvlt::error::PassStoreError;

#[derive(derive_more::From, Debug, derive_more::Display)]
pub enum Error {
    HomeNotFound,
    Io(std::io::Error),
    TomlDeserialize(toml::de::Error),
    TomlSerialize(toml::ser::Error),
    PassStore(PassStoreError)
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Error::PassStore(PassStoreError::GeneralError(error))
    }
}
