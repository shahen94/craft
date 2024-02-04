use std::{env, path::PathBuf};

use async_trait::async_trait;

use crate::{
    cache::TMP_CACHE_FOLDER,
    contracts::{Pipe, PipeArtifact},
    errors::{ExecutionError, ZipError},
    logger::CraftLogger,
    tar::Gzip,
};

use super::artifacts::StoredArtifact;

pub struct ExtractorPipe {
    packages: Vec<StoredArtifact>,
}

impl ExtractorPipe {
    pub fn get_tmp_folder() -> PathBuf {
      let mut home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
      home.push_str(TMP_CACHE_FOLDER);

      PathBuf::from(home)
    }

    pub fn new(artifacts: &dyn PipeArtifact<Vec<StoredArtifact>>) -> Self {
        Self {
            packages: artifacts.get_artifacts(),
        }
    }

    pub async fn unzip_archive(&self, artifact: &StoredArtifact) -> Result<(), ZipError> {
        let artifact = artifact.clone();

        tokio::task::spawn_blocking(move || {
            let tmp_folder = Self::get_tmp_folder();

            let dest = tmp_folder.join(format!(
                "{}-{}",
                artifact.package.name, artifact.package.version
            ));

            match Gzip::extract(&artifact.zip_path, &dest) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        })
        .await
        .unwrap()
    }
}

#[async_trait]
impl Pipe<()> for ExtractorPipe {
    async fn run(&mut self) -> Result<(), ExecutionError> {
        for artifact in &self.packages {
            CraftLogger::verbose(format!(
                "Extracting artifact: {}",
                artifact.package.to_string()
            ));
            self.unzip_archive(artifact).await.unwrap();
        }

        let tmp_folder = Self::get_tmp_folder();

        if tmp_folder.exists() {
            CraftLogger::verbose("Cleaning up temporary cache folder");
            tokio::fs::remove_dir_all(&tmp_folder).await.unwrap();
        }

        Ok(())
    }
}
