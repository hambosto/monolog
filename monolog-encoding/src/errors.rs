use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReedSolomonError {
    #[error("Invalid shard count: {0}")]
    InvalidShardCount(String),

    #[error("Invalid data size: {0}")]
    InvalidDataSize(String),

    #[error("Codec error: {0}")]
    CodecError(String),

    #[error("Encoding error: {0}")]
    EncodingError(String),

    #[error("Decoding error: {0}")]
    DecodingError(String),
}
