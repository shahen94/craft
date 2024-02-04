use crate::{
    command::{Command, SubCommand}, contracts::Pipe, errors::ExecutionError, logger::CraftLogger, pipeline::{CacheCleanPipe, DownloaderPipe, ExtractorPipe, LinkerPipe, ResolverPipe}
};

pub struct Program;

impl Program {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn execute(&mut self, args: Command) -> Result<(), ExecutionError> {
        if args.command.is_none() {
            todo!("Read package.json and install dependencies");
        }

        let command = args.command.unwrap();

        match command {
            SubCommand::Install(args) => {
                CraftLogger::verbose("\n\n\nResolving dependencies");
                let artifacts = ResolverPipe::new(args.package).run().await?;

                CraftLogger::verbose("\n\n\nDownloading dependencies");
                let artifacts = DownloaderPipe::new(&artifacts).run().await?;

                CraftLogger::verbose("\n\n\nExtracting dependencies");
                ExtractorPipe::new(&artifacts).run().await?;

                LinkerPipe::new().run().await?;

                return Ok(());
            }
            SubCommand::Cache(args) => {
                let _ = CacheCleanPipe::new(args).run().await;

                return Ok(());
            }
        }
    }
}
