pub mod codec;
pub mod errors;
pub mod processor;

pub use codec::ReedSolomonCodec;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_roundtrip() {
        let codec: ReedSolomonCodec = ReedSolomonCodec::new(10, 4).unwrap();
        let data: &[u8; 13] = b"Hello, World!";

        let encoded: Vec<u8> = codec.encode(data).unwrap();
        let decoded: Vec<u8> = codec.decode(&encoded).unwrap();

        assert_eq!(data.to_vec(), decoded);
    }

    #[test]
    fn test_invalid_shard_count() {
        assert!(ReedSolomonCodec::new(0, 1).is_err());
    }
}
