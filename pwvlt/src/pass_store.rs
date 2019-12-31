use crate::error::PassStoreError;

pub trait PassStore {
    fn password(&self, service: &str, username: &str) -> Result<String, PassStoreError>;

    fn set_password(
        &self,
        service: &str,
        username: &str,
        password: &str,
    ) -> Result<(), PassStoreError>;

    fn log_error(&self, err: PassStoreError);

    fn name(&self) -> &'static str;
}
