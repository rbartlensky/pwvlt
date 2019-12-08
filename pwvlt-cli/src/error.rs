#[derive(derive_error::Error, Debug)]
pub enum Error {
    HomeNotFound,
    Io(std::io::Error),
    TomlDeserialize(toml::de::Error),
    TomlSerialize(toml::ser::Error)
}
