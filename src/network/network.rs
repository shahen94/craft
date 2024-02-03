use std::path::PathBuf;

use tokio::io::AsyncWriteExt;
use tokio::fs::File;

use crate::errors::NetworkError;

pub struct Network;

impl Network {
  pub async fn download_file(url: &str, path: &PathBuf) -> Result<(), NetworkError> {
    let mut response = reqwest::get(url).await.unwrap();

    let mut file = File::create(path).await?;

    while let Some(chunk) = response.chunk().await? {
      file.write_all(&chunk).await?;
    }

    Ok(())
  }
}