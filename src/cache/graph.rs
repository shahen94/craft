use std::collections::{HashMap, HashSet};

use async_trait::async_trait;

use crate::contracts::InMemoryCache;

/// Dependency graph is a representation of a packages and it's dependencies
/// Since multiple packages can have the same dependency, we'll need to identify them
/// by their name and version. so we'll not need to fetch the same dependency multiple times.
/// TODO: Add Support for Lockfile
#[derive(Debug)]
pub struct DependencyGraph {
  pub packages: HashSet<String>,
  pub dependencies: HashMap<String, Vec<String>>,
}

impl DependencyGraph {
  pub fn new() -> Self {
    Self {
      packages: HashSet::new(),
      dependencies: HashMap::new(),
    }
  }
}

#[async_trait]
impl InMemoryCache<String> for DependencyGraph {
  async fn get(&self, key: &str) -> Option<String> {
    self.packages.get(key).cloned()
  }
  async fn set(&mut self, _: &str, value: String) -> () {
    self.packages.insert(value);
  }
}