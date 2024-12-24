use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Input file does not exist")]
    InputFileNotFound,

    #[error("Input file is empty")]
    EmptyInputFile,

    #[error("Output file already exists")]
    OutputFileExists,

    #[error("Invalid delete type")]
    InvalidDeleteType,

    #[error("Passwords do not match")]
    PasswordMismatch,

    #[error("Failed to get user input: {0}")]
    UserInput(String),

    #[error("{0}")]
    Other(String),
}
