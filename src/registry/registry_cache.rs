use std::collections::HashMap;

use crate::common::remote_package::RemotePackage;

#[derive(Debug)]
pub struct RegistryCache {
  registry: HashMap<String, RemotePackage>
}

impl RegistryCache {
  pub fn new() -> Self {
    Self {
      registry: HashMap::new()
    }
  }

  pub fn get_package(&self, package_name: &str) -> Option<&RemotePackage> {
    self.registry.get(package_name)
  }

  pub fn add_package(&mut self, package: &RemotePackage) {
    self.registry.insert(package.name.clone(), package.clone());
  }

  pub fn has_package(&self, package_name: &str) -> bool {
    self.registry.contains_key(package_name)
  }

  pub fn remove_package(&mut self, package_name: &str) {
    self.registry.remove(package_name);
  }

  pub fn clear(&mut self) {
    self.registry.clear();
  }
}