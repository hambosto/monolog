use crate::errors::FileError;
use rand::{thread_rng, RngCore};
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

const BUFFER_SIZE: usize = 4096;
const OVERWRITE_PASSES: usize = 3;

pub fn wipe_file<P: AsRef<Path>>(path: P) -> Result<(), FileError> {
    let path: &Path = path.as_ref();

    for pass in 0..OVERWRITE_PASSES {
        let mut file: File = OpenOptions::new()
            .write(true)
            .open(path)
            .map_err(|e| FileError::Io(e))?;

        let file_size: u64 = file.metadata().map_err(|e| FileError::Io(e))?.len();

        secure_overwrite(&mut file, file_size).map_err(|e| {
            FileError::Other(format!("secure overwrite pass {} failed: {}", pass + 1, e))
        })?;
    }

    std::fs::remove_file(path).map_err(|e| FileError::Io(e))?;
    Ok(())
}

fn secure_overwrite(file: &mut File, file_size: u64) -> Result<(), FileError> {
    file.seek(SeekFrom::Start(0)).map_err(FileError::Io)?;

    let mut rng = thread_rng();
    let mut buffer: Vec<u8> = vec![0u8; BUFFER_SIZE];
    let mut remaining: u64 = file_size;

    while remaining > 0 {
        let write_size = std::cmp::min(remaining, BUFFER_SIZE as u64) as usize;
        rng.fill_bytes(&mut buffer[..write_size]);

        file.write_all(&buffer[..write_size])
            .map_err(FileError::Io)?;
        remaining -= write_size as u64;
    }

    file.sync_all().map_err(FileError::Io)?;
    Ok(())
}
