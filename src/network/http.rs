use std::path::PathBuf;

use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use sha1::{Sha1, Digest};
use crate::errors::NetworkError;

pub struct Http;

impl Http {
    pub async fn download_file(url: &str, path: &PathBuf, sha_sum: &str) -> Result<(),
        NetworkError> {
        let mut response = reqwest::get(url).await?;
        let mut hasher = Sha1::new();

        let mut file = match File::create(path).await {
            Ok(file) => file,
            Err(_) => {
                println!("Could not create file at {:?}", path);
                panic!("Stop");
            }
        };

        while let Some(chunk) = response.chunk().await? {
            file.write_all(&chunk).await?;
            hasher.update(&chunk);
        }
        let calculated_sha = hasher.finalize().0;
        let calculated_sha_hex = hex::encode(calculated_sha);
        if calculated_sha_hex != sha_sum {
            return Err(NetworkError::CheckSum(url.to_string()));
        }
        Ok(())
    }
}
