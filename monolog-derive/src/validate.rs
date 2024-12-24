use crate::config::SALT_SIZE;
use crate::errors::KdfError;

pub(crate) fn validate_password(password: &[u8]) -> Result<(), KdfError> {
    if password.is_empty() {
        return Err(KdfError::EmptyPassword);
    }

    Ok(())
}

pub(crate) fn validate_salt(salt: &[u8]) -> Result<(), KdfError> {
    if salt.len() != SALT_SIZE {
        return Err(KdfError::InvalidSaltLength);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_password() {
        assert!(validate_password(b"password").is_ok());
        assert!(validate_password(b"").is_err());
    }

    #[test]
    fn test_validate_salt() {
        let valid_salt = vec![0u8; SALT_SIZE];
        let invalid_salt = vec![0u8; SALT_SIZE - 1];

        assert!(validate_salt(&valid_salt).is_ok());
        assert!(validate_salt(&invalid_salt).is_err());
    }
}
