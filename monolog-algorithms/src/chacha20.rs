use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use rand::{rngs::OsRng, RngCore};

use crate::errors::CryptoError;
use crate::Cipher;

pub struct ChaCha20Cipher {
    key: Vec<u8>,
    nonce: Vec<u8>,
}

impl ChaCha20Cipher {
    pub fn new(key: &[u8]) -> Result<Self, CryptoError> {
        if key.len() != 32 {
            return Err(CryptoError::InvalidKeySize(key.len()));
        }

        let mut nonce = vec![0u8; 12];
        OsRng.fill_bytes(&mut nonce);

        Ok(Self {
            key: key.to_vec(),
            nonce,
        })
    }
}

impl Cipher for ChaCha20Cipher {
    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if plaintext.is_empty() {
            return Err(CryptoError::EmptyPlaintext);
        }

        let cipher = ChaCha20Poly1305::new_from_slice(&self.key)
            .map_err(|e| CryptoError::EncryptionError(e.to_string()))?;

        let nonce = Nonce::from_slice(&self.nonce);

        cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| CryptoError::EncryptionError(e.to_string()))
    }

    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let cipher = ChaCha20Poly1305::new_from_slice(&self.key)
            .map_err(|e| CryptoError::DecryptionError(e.to_string()))?;

        let nonce = Nonce::from_slice(&self.nonce);

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| CryptoError::DecryptionError(e.to_string()))
    }

    fn set_nonce(&mut self, nonce: &[u8]) -> Result<(), CryptoError> {
        if nonce.len() != 12 {
            return Err(CryptoError::InvalidNonceSize(nonce.len()));
        }

        self.nonce = nonce.to_vec();

        Ok(())
    }

    fn get_nonce(&self) -> &[u8] {
        &self.nonce
    }
}
