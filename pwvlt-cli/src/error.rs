use pwvlt::error::PwvltError;

#[derive(derive_more::From, Debug, derive_more::Display)]
pub enum Error {
    HomeNotFound,
    Io(std::io::Error),
    TomlDeserialize(toml::de::Error),
    TomlSerialize(toml::ser::Error),
    PassStore(PwvltError),
    General(Box<dyn std::error::Error>)
}
