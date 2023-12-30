use std::{path::PathBuf, fs::File};

use flate2::read::GzDecoder;
use tar::Archive;

use super::errors::UnzipError;

pub struct Gzip;

impl Gzip {
    pub async fn extract(source: &PathBuf, dest: &PathBuf) -> Result<(), UnzipError> {
      let file = File::open(source.clone()).map_err(|err| {
        let error_msg = format!("Error opening file: {:?}", err);
        UnzipError::new(error_msg)
    })?;

      let tar = GzDecoder::new(file);
      let mut archive = Archive::new(tar);

      match archive.unpack(&dest) {
          Ok(_) => {}
          Err(error) => {
              let error_msg = format!("Error unpacking file: {:?}", error);
              return Err(UnzipError::new(error_msg));
          }
      };

      Ok(())
    }
}