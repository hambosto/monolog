use crate::errors::ReedSolomonError;
use byteorder::{BigEndian, ByteOrder};

pub struct DataProcessor;

impl DataProcessor {
    pub fn prepare_data(data: &[u8]) -> Result<Vec<u8>, ReedSolomonError> {
        let mut buffer = Vec::with_capacity(data.len() + 4);
        let mut size_prefix = [0u8; 4];
        BigEndian::write_u32(&mut size_prefix, data.len() as u32);
        buffer.extend_from_slice(&size_prefix);
        buffer.extend_from_slice(data);
        Ok(buffer)
    }

    pub fn split_into_shards(
        data: &[u8],
        data_shards: usize,
        total_shards: usize,
    ) -> Result<Vec<Vec<u8>>, ReedSolomonError> {
        let shard_size = (data.len() + data_shards - 1) / data_shards;
        let mut shards = vec![vec![0u8; shard_size]; total_shards];

        for (i, chunk) in data.chunks(shard_size).enumerate().take(data_shards) {
            shards[i][..chunk.len()].copy_from_slice(chunk);
        }

        Ok(shards)
    }

    pub fn validate_and_split_shares(
        data: &[u8],
        total_shards: usize,
    ) -> Result<Vec<Vec<u8>>, ReedSolomonError> {
        if data.is_empty() {
            return Err(ReedSolomonError::InvalidDataSize("Empty data".to_string()));
        }

        if data.len() % total_shards != 0 {
            return Err(ReedSolomonError::InvalidDataSize(format!(
                "Data length ({}) not divisible by total shards ({})",
                data.len(),
                total_shards
            )));
        }

        let share_size: usize = data.len() / total_shards;
        Ok((0..total_shards)
            .map(|i: usize| data[i * share_size..(i + 1) * share_size].to_vec())
            .collect())
    }

    pub fn extract_original_data(decoded: &[u8]) -> Result<Vec<u8>, ReedSolomonError> {
        if decoded.len() < 4 {
            return Err(ReedSolomonError::DecodingError(
                "Data too short".to_string(),
            ));
        }

        let original_size = BigEndian::read_u32(&decoded[..4]) as usize;
        if original_size > decoded.len() - 4 {
            return Err(ReedSolomonError::DecodingError(
                "Invalid size prefix".to_string(),
            ));
        }

        Ok(decoded[4..4 + original_size].to_vec())
    }
}
