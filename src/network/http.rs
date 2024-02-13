use std::path::PathBuf;

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::errors::NetworkError;

pub struct Http;

impl Http {
    pub async fn download_file(url: &str, path: &PathBuf) -> Result<(), NetworkError> {
        let mut response = reqwest::get(url).await.unwrap();

        let mut file = match File::create(path).await {
            Ok(file) => file,
            Err(_) => {
                println!("Could not create file at {:?}", path);
                panic!("Stop");
            }
        };

        while let Some(chunk) = response.chunk().await? {
            file.write_all(&chunk).await?;
        }

        Ok(())
    }
}
