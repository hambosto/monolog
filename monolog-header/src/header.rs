use crate::constants::{NONCE_SIZE, SALT_SIZE};
use crate::errors::HeaderError;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct FileHeader {
    pub salt: Vec<u8>,
    pub original_size: u64,
    pub aes_nonce: Vec<u8>,
    pub chacha20_nonce: Vec<u8>,
}

impl FileHeader {
    pub fn new(original_size: u64) -> Self {
        Self {
            salt: vec![0u8; SALT_SIZE],
            original_size,
            aes_nonce: vec![0u8; NONCE_SIZE],
            chacha20_nonce: vec![0u8; NONCE_SIZE],
        }
    }

    pub fn read<R: Read>(mut reader: R) -> Result<Self, HeaderError> {
        let mut header = Self::new(0);

        reader
            .read_exact(&mut header.salt)
            .map_err(HeaderError::SaltRead)?;

        header.original_size = reader
            .read_u64::<BigEndian>()
            .map_err(HeaderError::OriginalSizeRead)?;

        reader
            .read_exact(&mut header.aes_nonce)
            .map_err(HeaderError::SerpentNonceRead)?;

        reader
            .read_exact(&mut header.chacha20_nonce)
            .map_err(HeaderError::ChaCha20NonceRead)?;

        Ok(header)
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), HeaderError> {
        writer.write_all(&self.salt)?;
        writer.write_u64::<BigEndian>(self.original_size)?;
        writer.write_all(&self.aes_nonce)?;
        writer.write_all(&self.chacha20_nonce)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_header_roundtrip() {
        let original_header = FileHeader {
            salt: vec![1; SALT_SIZE],
            original_size: 12345,
            aes_nonce: vec![2; NONCE_SIZE],
            chacha20_nonce: vec![3; NONCE_SIZE],
        };

        let mut buffer = Vec::new();
        original_header.write(&mut buffer).unwrap();

        let mut cursor = Cursor::new(buffer);
        let read_header = FileHeader::read(&mut cursor).unwrap();

        assert_eq!(original_header, read_header);
    }
}
