use std::{
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

use crate::{
    command::{Command, SubCommand},
    contracts::{Pipe, Progress, ProgressAction},
    errors::ExecutionError,
    logger::CraftLogger,
    pipeline::{CacheCleanPipe, DownloaderPipe, ExtractorPipe, LinkerPipe, ResolverPipe},
    ui::UIProgress,
};

pub struct Program;

impl Program {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn start_progress(&self, rx: Receiver<ProgressAction>) -> JoinHandle<()> {
        thread::spawn(move || {
          let progress = UIProgress::new();

          progress.start(rx);
        })
    }

    pub async fn execute(&mut self, args: Command) -> Result<(), ExecutionError> {
        if args.command.is_none() {
            todo!("Read package.json and install dependencies");
        }

        let command = args.command.unwrap();

        match command {
            SubCommand::Install(args) => {
                let (tx, rx) = std::sync::mpsc::channel();

                let ui_thread = self.start_progress(rx);

                CraftLogger::verbose_n(3, "Resolving dependencies");
                let resolve_artifacts = ResolverPipe::new(args.package, tx.clone()).run().await?;

                CraftLogger::verbose_n(3, "Downloading dependencies");
                let download_artifacts = DownloaderPipe::new(&resolve_artifacts, tx.clone())
                    .run()
                    .await?;

                CraftLogger::verbose_n(3, "Extracting dependencies");
                let extracted_artifacts = ExtractorPipe::new(&download_artifacts, tx.clone())
                    .run()
                    .await?;

                CraftLogger::verbose_n(3, "Linking dependencies");
                LinkerPipe::new(tx.clone()).run().await?;

                drop(tx);
                ui_thread.join().unwrap();
                return Ok(());
            }
            SubCommand::Cache(args) => {
                let _ = CacheCleanPipe::new(args).run().await;

                return Ok(());
            }
        }
    }
}
