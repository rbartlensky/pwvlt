use crate::error::PassStoreError;

use passwords::PasswordGenerator;

pub fn random_password() -> Result<String, PassStoreError> {
    let pg = PasswordGenerator {
        length: 22,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        strict: true,
    };
    pg.generate_one()
        .map_err(|e| PassStoreError::PasswordGenerationError(e.into()))
}
