use crate::config::{ARGON_MEM_COST, ARGON_THREADS, ARGON_TIME_COST, KEY_SIZE};
use crate::errors::KdfError;
use crate::validate::{validate_password, validate_salt};
use argon2::{Argon2, Version};

pub fn derive(password: &[u8], salt: &[u8]) -> Result<Vec<u8>, KdfError> {
    validate_password(password)?;
    validate_salt(salt)?;

    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        Version::V0x13,
        argon2::Params::new(
            ARGON_MEM_COST,
            ARGON_TIME_COST,
            ARGON_THREADS,
            Some(KEY_SIZE),
        )
        .map_err(|e| KdfError::DerivationError(e.to_string()))?,
    );

    let mut key = vec![0u8; KEY_SIZE];
    argon2
        .hash_password_into(password, salt, &mut key)
        .map_err(|e| KdfError::DerivationError(e.to_string()))?;

    Ok(key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SALT_SIZE;
    use crate::salt::generate_salt;

    #[test]
    fn test_derive_with_valid_input() {
        let password = b"test_password";
        let salt = generate_salt().unwrap();
        let key = derive(password, &salt).unwrap();
        assert_eq!(key.len(), KEY_SIZE);
    }

    #[test]
    fn test_empty_password() {
        let password = b"";
        let salt = generate_salt().unwrap();
        assert!(matches!(
            derive(password, &salt),
            Err(KdfError::EmptyPassword)
        ));
    }

    #[test]
    fn test_invalid_salt_length() {
        let password = b"test_password";
        let salt = vec![0u8; SALT_SIZE - 1]; // Invalid salt length
        assert!(matches!(
            derive(password, &salt),
            Err(KdfError::InvalidSaltLength)
        ));
    }
}
