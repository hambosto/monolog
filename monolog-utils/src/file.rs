use crate::errors::FileError;
use crate::prompt::DeleteType;
use crate::wipe::wipe_file;
use std::fs::File;
use std::path::Path;

pub fn delete_file<P: AsRef<Path>>(file_path: P, delete_type: DeleteType) -> Result<(), FileError> {
    match delete_type {
        DeleteType::Normal => std::fs::remove_file(file_path).map_err(FileError::Io),
        DeleteType::Secure => wipe_file(file_path),
    }
}

pub fn prepare_output_file<P: AsRef<Path>>(output_file: P) -> Result<File, FileError> {
    File::create(output_file).map_err(|e| FileError::Io(e))
}
