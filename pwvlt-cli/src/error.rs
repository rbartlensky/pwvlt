#[derive(derive_error::Error, Debug)]
pub enum Error {
    HomeNotFound,
    Io(std::io::Error),
    Toml(toml::de::Error),
}
