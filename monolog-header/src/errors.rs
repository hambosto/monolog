use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HeaderError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Failed to read salt: {0}")]
    SaltRead(io::Error),

    #[error("Failed to read original size: {0}")]
    OriginalSizeRead(io::Error),

    #[error("Failed to read serpent nonce: {0}")]
    SerpentNonceRead(io::Error),

    #[error("Failed to read chacha20 nonce: {0}")]
    ChaCha20NonceRead(io::Error),
}
