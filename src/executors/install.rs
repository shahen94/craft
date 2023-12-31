use std::collections::HashMap;

use async_trait::async_trait;
use futures::StreamExt;

use crate::{
    common::{
        contracts::{Actor, Modules, Registry},
        errors::{InstallError, UninstallError},
        package::Package,
    },
    fs::{NodeModules, Project},
    logger::CraftLogger,
    registry::NpmRegistry,
};

#[derive(Debug)]
pub struct InstallActions {
    registry: NpmRegistry,
    pub modules: NodeModules,
}

impl InstallActions {
    pub fn new(base_dir: Option<&str>) -> Self {
        let base_dir = match base_dir {
            Some(base_dir) => base_dir,
            None => "./node_modules",
        };
        Self {
            registry: NpmRegistry::new(None),
            modules: NodeModules::new(base_dir),
        }
    }

    pub async fn clean_cache(&self) {
        let _ = self.modules.cache.force_clean().await;
    }
}

#[async_trait]
impl Actor for InstallActions {
    async fn install_package(&mut self, package: &Package) -> Result<(), InstallError> {
        let is_installed = self.modules.is_package_installed(&package).await;

        if is_installed {
            let msg = format!("{}@{} already installed", package.name, package.version);

            CraftLogger::log(msg);
            return Ok(());
        }

        let result = self.registry.get_package(package).await;

        if result.is_err() {
            return Err(InstallError::new(result.err().unwrap().reason));
        }

        let result = result.unwrap();
        self.modules
            .download_package(&result)
            .await
            .map_err(|err| InstallError::new(err.reason))?;

        match self.modules.unzip_package(&result).await {
            Ok(_) => {}
            Err(err) => {
                return Err(InstallError::new(err.reason));
            }
        }

        // Recursively call install_package for each dependency

        let dependencies = result
            .dependencies
            .iter()
            .chain(result.dev_dependencies.iter())
            .collect::<HashMap<&String, &String>>();

        for (name, version) in dependencies.iter() {
            let package = Package::new((**name).clone(), (**version).clone()).unwrap();
            self.install_package(&package).await?;
        }
        
        let msg = format!("{}@{} installed", package.name, package.version);
        CraftLogger::info(msg);

        Ok(())
    }

    async fn uninstall_package(&self, package: &Package) -> Result<(), UninstallError> {
        self.modules
            .remove_package(&package.name)
            .await
            .map_err(|err| UninstallError::new(err.reason))?;

        Ok(())
    }

    async fn update_package(&self, package: &Package) {}

    async fn list_packages(&self) {}

    async fn install_all_packages(&self) {
        let project = Project::new(None).await.unwrap();
        self.modules.cleanup().await;

        // let mut tasks = Vec::new();

        let dependencies = project
            .dependencies
            .iter()
            .chain(project.dev_dependencies.iter())
            .collect::<HashMap<&String, &String>>();

        // Use futures::stream to run tasks in parallel

        futures::stream::iter(dependencies.iter())
            .for_each_concurrent(10, |(name, version)| async move {
                let package = Package::new((**name).clone(), (**version).clone()).unwrap();

                let mut actions = InstallActions::new(None);
                let data = actions.install_package(&package).await;

                if data.is_err() {
                    let msg = format!("{:?}", data.err().unwrap().reason);
                    CraftLogger::error(msg)
                }
            })
            .await;

        // for (name, version) in dependencies.iter() {
        //     let package = Package::new((**name).clone(), (**version).clone()).unwrap();

        //     let mut actions = InstallActions::new(None);
        //     let task = tokio::spawn(async move {
        //         let data = actions.install_package(&package).await;

        //         if data.is_err() {
        //             let msg = format!("{:?}", data.err().unwrap().reason);
        //             CraftLogger::error(msg)
        //         }
        //     });

        //     tasks.push(task);
        // }

        // for task in tasks {
        //     task.await.unwrap();
        // }
    }
}
