use std::sync::{mpsc::Sender, Arc};

use async_trait::async_trait;
use tokio::fs;
use tokio::sync::Mutex;

use crate::{
    contracts::{Phase, Pipe, PipeArtifact, ProgressAction},
    errors::{ExecutionError, ZipError},
    logger::CraftLogger,
    tar::Gzip,
};
use crate::pipeline::ResolvedItem;
use super::artifacts::{ExtractArtifacts, StoredArtifact};

pub struct ExtractorPipe {
    packages: Vec<StoredArtifact>,
    artifacts: Arc<Mutex<ExtractArtifacts>>,

    tx: Sender<ProgressAction>,
}

impl ExtractorPipe {
    pub fn new(
        artifacts: &dyn PipeArtifact<Vec<StoredArtifact>>,
        tx: Sender<ProgressAction>,
    ) -> Self {
        Self {
            packages: artifacts.get_artifacts(),
            artifacts: Arc::new(Mutex::new(ExtractArtifacts::new())),
            tx,
        }
    }

    // Skip because we now simlink the extracted files
    pub async fn cleanup(vec: Vec<ResolvedItem>) -> Result<(), ExecutionError> {
        let mapped_str = vec.iter().map(|x| x.package.name.clone()).collect::<Vec<String>>();
        let mut entries = tokio::fs::read_dir("node_modules").await.map_err
        (|e|ExecutionError::JobExecutionFailed(e.to_string(),e.to_string()))?;
       while let Some(entry) = entries.next_entry().await.map_err(|e|ExecutionError::JobExecutionFailed(e.to_string(),e.to_string()))? {
            let dir_name = entry.file_name().to_str().unwrap().to_string();
            if !mapped_str.contains(&dir_name) {
                fs::remove_dir_all(entry.path()).await.map_err(|e|ExecutionError::JobExecutionFailed(e.to_string(),e.to_string()))?;
            }
        }

        Ok(())
    }

    pub async fn unzip_archive(&self, artifact: &StoredArtifact) -> Result<(), ZipError> {
        let artifact_s = artifact.clone();

        let tmp_folder = ExtractArtifacts::get_tmp_folder();



        tokio::task::spawn_blocking(move || {
            let dest = tmp_folder.join(format!(
                "{}-{}",
                &artifact_s.package.name, &artifact_s.package.version
            ));

            // Skip if already unzipped
            if dest.exists() {
                return Ok(());
            }
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
                artifact.package
            ));
            self.unzip_archive(artifact).await.unwrap();
        }

        Ok(self.artifacts.lock().await.clone())
    }
}
