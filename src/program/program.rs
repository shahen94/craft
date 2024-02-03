use crate::{
    command::{Command, SubCommand},
    contracts::Pipe,
    errors::ExecutionError,
    pipeline::{CacheCleanPipe, DownloaderPipe, ExtractorPipe, LinkerPipe, ResolverPipe},
};

pub struct Program;

impl Program {
    pub fn new() -> Self {
        Self {
            
        }
    }

    pub async fn execute(&mut self, args: Command) -> Result<(), ExecutionError> {
        if args.command.is_none() {
            todo!("Read package.json and install dependencies");
        }

        let command = args.command.unwrap();

        match command {
            SubCommand::Install(args) => {
                let artifacts = ResolverPipe::new(args.package).run().await?;

                DownloaderPipe::new(&artifacts)
                    .run()
                    .await?;

                ExtractorPipe::new()
                    .run()
                    .await?;

                LinkerPipe::new()
                    .run()
                    .await?;

                return Ok(());
            }
            SubCommand::Cache(args) => {
                let _ = CacheCleanPipe::new(args).run().await;

                return Ok(());
            }
        }
    }
}
