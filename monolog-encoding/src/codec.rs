use crate::errors::ReedSolomonError;
use crate::processor::DataProcessor;
use reed_solomon_erasure::galois_8::ReedSolomon;

pub struct ReedSolomonCodec {
    codec: ReedSolomon,
    data_shards: usize,
    total_shards: usize,
}

impl ReedSolomonCodec {
    pub fn new(data_shards: usize, parity_shards: usize) -> Result<Self, ReedSolomonError> {
        if data_shards == 0 || parity_shards == 0 {
            return Err(ReedSolomonError::InvalidShardCount(
                "Shard counts must be greater than zero".to_string(),
            ));
        }

        let total_shards: usize = data_shards + parity_shards;
        let codec = ReedSolomon::new(data_shards, parity_shards)
            .map_err(|e| ReedSolomonError::CodecError(e.to_string()))?;

        Ok(Self {
            codec,
            data_shards,
            total_shards,
        })
    }

    pub fn encode(&self, data: &[u8]) -> Result<Vec<u8>, ReedSolomonError> {
        let encoded_data: Vec<u8> = DataProcessor::prepare_data(data)?;
        let mut shards: Vec<Vec<u8>> =
            DataProcessor::split_into_shards(&encoded_data, self.data_shards, self.total_shards)?;

        let mut shard_refs: Vec<&mut [u8]> = shards.iter_mut().map(Vec::as_mut_slice).collect();
        self.codec
            .encode(&mut shard_refs)
            .map_err(|e| ReedSolomonError::EncodingError(e.to_string()))?;

        Ok(shards.into_iter().flatten().collect())
    }

    pub fn decode(&self, data: &[u8]) -> Result<Vec<u8>, ReedSolomonError> {
        let shares: Vec<Vec<u8>> =
            DataProcessor::validate_and_split_shares(data, self.total_shards)?;
        let shard_size: usize = shares[0].len();

        let mut decode_buffer: Vec<u8> = vec![0u8; shard_size * self.data_shards];
        let mut decode_shards: Vec<_> = decode_buffer.chunks_mut(shard_size).collect();

        for (i, share) in shares.iter().take(self.data_shards).enumerate() {
            decode_shards[i].copy_from_slice(share);
        }

        DataProcessor::extract_original_data(&decode_buffer)
    }
}
