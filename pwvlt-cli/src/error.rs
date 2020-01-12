use pwvlt::PwvltError;

#[derive(derive_more::From, Debug, derive_more::Display)]
pub enum Error {
    HomeNotFound,
    Io(std::io::Error),
    TomlDeserialize(toml::de::Error),
    TomlSerialize(toml::ser::Error),
    Pwvlt(PwvltError),
    General(Box<dyn std::error::Error>)
}
