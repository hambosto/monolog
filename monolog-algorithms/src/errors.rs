use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Invalid key size: {0} bytes")]
    InvalidKeySize(usize),
    #[error("Invalid nonce size: {0} bytes")]
    InvalidNonceSize(usize),
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    #[error("Decryption error: {0}")]
    DecryptionError(String),
    #[error("Empty plaintext")]
    EmptyPlaintext,
}
