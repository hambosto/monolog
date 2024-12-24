use crate::errors::FileError;
use std::path::Path;

pub fn validate_input_file<P: AsRef<Path>>(input_file: P) -> Result<(), FileError> {
    let metadata = std::fs::metadata(input_file).map_err(|_| FileError::InputFileNotFound)?;

    if metadata.len() == 0 {
        return Err(FileError::EmptyInputFile);
    }

    Ok(())
}

pub fn validate_output_file<P: AsRef<Path>>(output_file: P) -> Result<(), FileError> {
    if output_file.as_ref().exists() {
        return Err(FileError::OutputFileExists);
    }
    Ok(())
}
