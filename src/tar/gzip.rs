use std::{
    fs::File,
    path::{Path, PathBuf},
};

use flate2::read::GzDecoder;
use tar::Archive;

use crate::errors::ZipError;

pub struct Gzip;

impl Gzip {
    pub fn extract(source: &Path, dest: &PathBuf) -> Result<(), ZipError> {
        let file = File::open(source)?;

        let tar = GzDecoder::new(file);
        let mut archive = Archive::new(tar);

        match archive.unpack(dest) {
            Ok(_) => {}
            Err(error) => {
                let error_msg = format!("Error unpacking file: {:?}", error);
                return Err(ZipError::FailedToUnzip(error_msg));
            }
        };

        Ok(())
    }
}
