use std::collections::HashMap;
use clap::builder::Str;
use crate::command::ProgramDesire;
use crate::errors::ExecutionError;
use crate::package::PackageJson;

pub struct PreprocessDependencyInstall{
    pub program_desire: ProgramDesire,
}


impl PreprocessDependencyInstall {
    pub fn new(program_desire: ProgramDesire) -> PreprocessDependencyInstall {
        PreprocessDependencyInstall{
            program_desire
        }
    }

    pub(crate) fn read_package_json() -> Result<PackageJson, ExecutionError> {
        std::fs::read_to_string("package.json").map(|e|e.into()).map_err(|_|
            ExecutionError::PackageJsonNotFound)
    }

    fn format_dependencies(&self, dependencies: HashMap<String, String>) -> Vec<String> {
        let mut packages = vec![];
        for (name, version) in dependencies {
            packages.push(format!("{}@{}", name, version));
        }
        packages
    }

    /// Calculates the main dependencies to use
    fn calculate_dependencies(&self) -> Result<Vec<String>, ExecutionError> {
        let mut dependencies = vec![];
        let package_json =  Self::read_package_json()?;
        if self.program_desire.dev_install {
            if let Some(dev_deps) = package_json.dev_dependencies {
                dependencies.append(&mut self.format_dependencies(dev_deps));
            }
        }

        if self.program_desire.prod_install {
            if let Some(prod_deps) = package_json.dependencies {
                dependencies.append(&mut self.format_dependencies(prod_deps));
            }
        }

        if self.program_desire.optional_install {
            if let Some(optional_deps) = package_json.optional_dependencies {
                dependencies.append(&mut self.format_dependencies(optional_deps));
            }
        }

        Ok(dependencies)

    }

    pub fn get_script() -> Result<HashMap<String, String>, ExecutionError> {
        let package_json = Self::read_package_json()?;
        Ok(package_json.scripts.unwrap_or(HashMap::new()))
    }

    pub async fn run(&self) -> Result<Vec<String>, ExecutionError> {
        let dependency_to_install = self.calculate_dependencies()?;

        Ok(dependency_to_install)
    }
}