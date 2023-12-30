use async_trait::async_trait;

use crate::{
    common::{
        contracts::{Actor, Modules, Registry},
        errors::{InstallError, UninstallError},
        package::Package,
    },
    fs::NodeModules,
    registry::NpmRegistry,
};

#[derive(Debug)]
pub struct InstallActions {
    registry: NpmRegistry,
    pub modules: NodeModules,
}

impl InstallActions {
    pub fn new() -> Self {
        Self {
            registry: NpmRegistry::new(None),
            modules: NodeModules::new("./node_modules"),
        }
    }
}

#[async_trait]
impl Actor for InstallActions {
    async fn install_package(&self, package: &Package) -> Result<(), InstallError> {
        let result = self.registry.get_package(package).await;

        if result.is_err() {
            return Err(InstallError::new(result.err().unwrap().reason));
        }

        let result = result.unwrap();
        self
            .modules
            .download_package(&result)
            .await
            .map_err(|err| InstallError::new(err.reason))?;

        match self.modules.unzip_package(&result).await {
            Ok(_) => {}
            Err(err) => {
                return Err(InstallError::new(err.reason));
            }
        }
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
      self.modules.cleanup().await;
    }
}
