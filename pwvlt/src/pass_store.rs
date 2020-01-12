use crate::PwvltError;

#[derive(Clone)]
pub struct Slot {
    pub service: String,
    pub username: String,
}

impl Default for Slot {
    fn default() -> Slot {
        Slot {
            service: "<not programmed>".into(),
            username: "<not programmed>".into(),
        }
    }
}

pub trait PassStore {
    fn password(&self, service: &str, username: &str) -> Result<String, PwvltError>;

    fn set_password(
        &self,
        slot: usize,
        service: &str,
        username: &str,
        password: &str,
    ) -> Result<(), PwvltError>;

    fn log_error(&self, err: PwvltError);

    fn name(&self) -> &'static str;

    fn slots(&self) -> Result<Vec<Slot>, PwvltError>;
}
