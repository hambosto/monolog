use crate::config::SALT_SIZE;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KdfError {
    #[error("Failed to generate salt: {0}")]
    SaltGeneration(String),

    #[error("Password cannot be empty")]
    EmptyPassword,

    #[error("Salt must be {SALT_SIZE} bytes")]
    InvalidSaltLength,

    #[error("Key derivation failed: {0}")]
    DerivationError(String),
}
