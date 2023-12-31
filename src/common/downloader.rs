use std::path::Path;

use tokio::io::AsyncWriteExt;
use tokio::fs::File;

use super::errors::GzipDownloadError;

pub struct Downloader;

impl Downloader {
  pub async fn download_file(url: &str, path: &Path) -> Result<(), GzipDownloadError> {
    let mut response = reqwest::get(url).await?;

    if !response.status().is_success() {
      return Err(GzipDownloadError::new(format!(
        "Failed to download file from {}",
        url
      )));
    }

    let mut file = File::create(path).await?;

    while let Some(chunk) = response.chunk().await? {
      file.write_all(&chunk).await?;
    }

    Ok(())
  }
}