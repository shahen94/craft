use std::process;

use crate::{
    cache::RegistryCache,
    cmd::{CacheAction, Command, Install, SubCommand},
    common::package::Package,
    executors::InstallActions,
    logger::CraftLogger,
};

pub struct Program {
    action: Command,
    registry_cache: RegistryCache,

    install_executor: InstallActions,
}

impl Program {
    pub async fn new(command: Command) -> Self {
        let registry_cache = RegistryCache::new(None);
        let install_executor = InstallActions::new(None);

        install_executor.init_directories().await;
        registry_cache.init_cache().await;

        Self {
            action: command,
            registry_cache,
            install_executor,
        }
    }

    pub async fn execute(&mut self) {
        let command = match self.action.clone().command {
            Some(command) => command,
            None => {
                // $ craft
                self.install_executor
                    .install_all_packages(&self.registry_cache)
                    .await;
                let _ = self.registry_cache.persist().await;
                return;
            }
        };

        match command {
            // $ craft cache clean
            SubCommand::Cache(action) => match action {
                CacheAction::Clean => {
                    self.registry_cache.clear().await;
                    self.install_executor.clean_cache().await;
                }
            },

            // $ craft install <package>
            SubCommand::Install(Install { package }) => {
                let (name, version) = Package::parse_package(package);
                let package = match Package::new(name, version) {
                    Ok(package) => package,
                    Err(err) => {
                        CraftLogger::error(err.reason);
                        process::exit(1);
                    }
                };

                match self
                    .install_executor
                    .install_package(&package, &self.registry_cache)
                    .await
                {
                    Ok(_) => {}
                    Err(err) => {
                        CraftLogger::error(err.reason);
                        process::exit(1);
                    }
                };
            }
        }
    }
}
