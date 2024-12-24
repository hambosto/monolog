use crate::config::SALT_SIZE;
use crate::errors::KdfError;
use argon2::password_hash::rand_core::{OsRng, RngCore};

pub fn generate_salt() -> Result<Vec<u8>, KdfError> {
    let mut salt = vec![0u8; SALT_SIZE];
    OsRng
        .try_fill_bytes(&mut salt)
        .map_err(|e| KdfError::SaltGeneration(e.to_string()))?;

    Ok(salt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_salt() {
        let salt = generate_salt().unwrap();
        assert_eq!(salt.len(), SALT_SIZE);
    }
}
