use std::{env, path::PathBuf, sync::{mpsc::Sender, Arc}};

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{
    cache::TMP_CACHE_FOLDER,
    contracts::{Phase, Pipe, PipeArtifact, ProgressAction},
    errors::{ExecutionError, ZipError},
    logger::CraftLogger,
    tar::Gzip,
};

use super::artifacts::{ExtractArtifacts, StoredArtifact};

pub struct ExtractorPipe {
    packages: Vec<StoredArtifact>,
    artifacts: Arc<Mutex<ExtractArtifacts>>,

    tx: Sender<ProgressAction>,
}

impl ExtractorPipe {
    pub fn new(artifacts: &dyn PipeArtifact<Vec<StoredArtifact>>, tx: Sender<ProgressAction>,) -> Self {
        Self {
            packages: artifacts.get_artifacts(),
            artifacts: Arc::new(Mutex::new(ExtractArtifacts::new())),
            tx
        }
    }

    pub async fn unzip_archive(&self, artifact: &StoredArtifact) -> Result<(), ZipError> {
        let artifact_s = artifact.clone();

        let tmp_folder = ExtractArtifacts::get_tmp_folder();

        tokio::task::spawn_blocking(move || {
            let dest = tmp_folder.join(format!(
                "{}-{}",
                &artifact_s.package.name, &artifact_s.package.version
            ));
            match Gzip::extract(&artifact_s.zip_path, &dest) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        })
        .await
        .unwrap()?;

        let extracted_at = ExtractArtifacts::get_tmp_folder().join(format!(
            "{}-{}",
            &artifact.package.name, &artifact.package.version
        ));

        self.artifacts
            .lock()
            .await
            .add(artifact.package.clone(), extracted_at);

        Ok(())
    }
}

#[async_trait]
impl Pipe<ExtractArtifacts> for ExtractorPipe {
    async fn run(&mut self) -> Result<ExtractArtifacts, ExecutionError> {
      let _ = self.tx.send(ProgressAction::new(Phase::Extracting));

        for artifact in &self.packages {
            CraftLogger::verbose(format!(
                "Extracting artifact: {}",
                artifact.package.to_string()
            ));
            self.unzip_archive(artifact).await.unwrap();
        }

        let tmp_folder = ExtractArtifacts::get_tmp_folder();

        if tmp_folder.exists() {
            CraftLogger::verbose("Cleaning up temporary cache folder");
            tokio::fs::remove_dir_all(&tmp_folder).await.unwrap();
        }

        Ok(self.artifacts.lock().await.clone())
    }
}
