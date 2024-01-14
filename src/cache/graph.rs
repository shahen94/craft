use std::collections::HashMap;

use async_trait::async_trait;

use crate::contracts::InMemoryCache;

/// Dependency graph is a representation of a packages and it's dependencies
/// Since multiple packages can have the same dependency, we'll need to identify them
/// by their name and version. so we'll not need to fetch the same dependency multiple times.
#[derive(Debug)]
pub struct DependencyGraph {
  pub packages: Vec<String>,
  pub dependencies: HashMap<String, Vec<String>>,
}

impl DependencyGraph {
  pub fn new() -> Self {
    Self {
      packages: Vec::new(),
      dependencies: HashMap::new(),
    }
  }
}

#[async_trait]
impl InMemoryCache<String> for DependencyGraph {
  async fn get(&self, key: &str) -> Option<String> {
    todo!()
  }
  async fn set(&self, key: &str, value: String) -> () {
    todo!()
  }
}