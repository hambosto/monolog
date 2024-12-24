pub mod aes;
pub mod chacha20;
pub mod errors;

pub use aes::AesGcmCipher;
pub use chacha20::ChaCha20Cipher;
pub use errors::CryptoError;

pub trait Cipher {
    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn set_nonce(&mut self, nonce: &[u8]) -> Result<(), CryptoError>;
    fn get_nonce(&self) -> &[u8];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chacha20_encryption_decryption() {
        let key = [1u8; 32];
        let plaintext = b"Hello, World!";

        let cipher = ChaCha20Cipher::new(&key).unwrap();
        let ciphertext = cipher.encrypt(plaintext).unwrap();
        let decrypted = cipher.decrypt(&ciphertext).unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_aes_gcm_encryption_decryption() {
        let key = [1u8; 32];
        let plaintext = b"Hello, World!";

        let cipher = AesGcmCipher::new(&key).unwrap();
        let ciphertext = cipher.encrypt(plaintext).unwrap();
        let decrypted = cipher.decrypt(&ciphertext).unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_invalid_key_size() {
        let key = [1u8; 16];
        assert!(ChaCha20Cipher::new(&key).is_err());
        assert!(AesGcmCipher::new(&key).is_err());
    }

    #[test]
    fn test_invalid_nonce_size() {
        let key = [1u8; 32];
        let invalid_nonce = [1u8; 16];

        let mut chacha = ChaCha20Cipher::new(&key).unwrap();
        assert!(chacha.set_nonce(&invalid_nonce).is_err());

        let mut aes = AesGcmCipher::new(&key).unwrap();
        assert!(aes.set_nonce(&invalid_nonce).is_err());
    }

    #[test]
    fn test_empty_plaintext() {
        let key = [1u8; 32];
        let empty_plaintext = b"";

        let chacha = ChaCha20Cipher::new(&key).unwrap();
        assert!(chacha.encrypt(empty_plaintext).is_err());

        let aes = AesGcmCipher::new(&key).unwrap();
        assert!(aes.encrypt(empty_plaintext).is_err());
    }
}
