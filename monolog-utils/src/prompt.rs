use crate::errors::FileError;
use inquire::{Confirm, Password, Select};

pub fn prompt_overwrite(output_file: &str) -> Result<bool, FileError> {
    Confirm::new(&format!(
        "Output file {} already exists. Overwrite?",
        output_file
    ))
    .with_default(false)
    .prompt()
    .map_err(|e| FileError::UserInput(e.to_string()))
}

pub fn prompt_password() -> Result<String, FileError> {
    let password = Password::new("Enter password:")
        .prompt()
        .map_err(|e| FileError::UserInput(e.to_string()))?;

    let confirm = Password::new("Confirm password:")
        .prompt()
        .map_err(|e| FileError::UserInput(e.to_string()))?;

    if password != confirm {
        return Err(FileError::PasswordMismatch);
    }

    Ok(password)
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeleteType {
    Normal,
    Secure,
}

impl DeleteType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DeleteType::Normal => "Normal delete (faster, but recoverable)",
            DeleteType::Secure => "Secure delete (slower, but unrecoverable)",
        }
    }
}

pub fn prompt_delete_file(
    file_type: &str,
    file_path: &str,
) -> Result<Option<DeleteType>, FileError> {
    let should_delete = Confirm::new(&format!("Delete {} file {}?", file_type, file_path))
        .with_default(false)
        .prompt()
        .map_err(|e| FileError::UserInput(e.to_string()))?;

    if !should_delete {
        return Ok(None);
    }

    let options = vec![DeleteType::Normal.as_str(), DeleteType::Secure.as_str()];
    let delete_type = Select::new("Select delete type:", options)
        .with_starting_cursor(0) // Changed from with_default to with_starting_cursor
        .prompt()
        .map_err(|e| FileError::UserInput(e.to_string()))?;

    Ok(Some(match delete_type {
        "Normal delete (faster, but recoverable)" => DeleteType::Normal,
        "Secure delete (slower, but unrecoverable)" => DeleteType::Secure,
        _ => return Err(FileError::InvalidDeleteType),
    }))
}
