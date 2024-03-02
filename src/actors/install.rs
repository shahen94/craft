use std::{
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

use async_trait::async_trait;

use crate::{
    contracts::{Actor, Pipe, PipeArtifact, Progress, ProgressAction},
    errors::ExecutionError,
    logger::CraftLogger,
    pipeline::{DownloaderPipe, ExtractorPipe, LinkerPipe, ResolverPipe},
    ui::UIProgress,
};

pub struct InstallActor {
    packages: Vec<String>,
}

impl InstallActor {
    pub fn new(packages: Vec<String>) -> Self {
        Self { packages }
    }

    fn start_progress(&self, rx: Receiver<ProgressAction>) -> JoinHandle<()> {
        thread::spawn(move || {
            let progress = UIProgress::default();

            progress.start(rx);
        })
    }
}

type PipeResult = Result<(), ExecutionError>;

#[async_trait]
impl Actor<PipeResult> for InstallActor {
    async fn start(&mut self) -> PipeResult {
        let (tx, rx) = std::sync::mpsc::channel();

        let ui_thread = self.start_progress(rx);

        // ─── Start Resolving ─────────────────────────

        CraftLogger::verbose_n(3, "Resolving dependencies");
        let resolve_artifacts = ResolverPipe::new(self.packages.clone(), tx.clone())
            .run()
            .await?;
        CraftLogger::verbose_n(
            3,
            format!("Resolved: {:?}", resolve_artifacts.get_artifacts().len()),
        );

        // ─── Start Downloading ──────────────────────

        CraftLogger::verbose_n(3, "Downloading dependencies");
        let download_artifacts = DownloaderPipe::new(&resolve_artifacts, tx.clone())
            .run()
            .await?;

        CraftLogger::verbose_n(
            3,
            format!("Downloaded {:?}", download_artifacts.get_artifacts().len()),
        );

        // ─── Start Extracting ───────────────────────

        CraftLogger::verbose_n(3, "Extracting dependencies");
        let extracted_artifacts = ExtractorPipe::new(&download_artifacts, tx.clone())
            .run()
            .await?;

        CraftLogger::verbose_n(
            3,
            format!("Extracted {:?}", extracted_artifacts.get_artifacts().len()),
        );

        // ─── Start Linking ──────────────────────────

        CraftLogger::verbose_n(3, "Linking dependencies");
        LinkerPipe::new(
            tx.clone(),
            resolve_artifacts.get_artifacts(),
            extracted_artifacts.get_artifacts(),
        )
        .run()
        .await?;

        // ─── Sync Lock File ────────────────────────
        // TODO: Sync lock file

        // ─── Cleanup ────────────────────────────────

        ExtractorPipe::cleanup().await;

        drop(tx);
        ui_thread.join().unwrap();
        Ok(())
    }
}
