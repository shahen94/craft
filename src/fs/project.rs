use std::{collections::HashMap, fs};

use serde::Deserialize;

use crate::common::errors::JsonError;


/// The project struct represents the package.json file
/// 
/// # Example
/// ```
/// let project = Project::new();
/// println!("{:?}", project);
/// ```
/// 
/// # Fields
/// * `dependencies` - A hashmap that holds the dependencies
/// * `dev_dependencies` - A hashmap that holds the dev dependencies
/// 
/// # Errors
/// * `JsonError` - If the package.json file could not be read
#[derive(Debug, Deserialize)]
pub struct Project {
  #[serde(default)]
  pub dependencies: HashMap<String, String>,

  #[serde(default)]
  #[serde(rename = "devDependencies")]
  pub dev_dependencies: HashMap<String, String>
}

impl Project {
  /// Returns a new instance of Project
  /// 
  /// # Example
  /// ```
  /// let project = Project::new();
  /// println!("{:?}", project);
  /// ```
  /// 
  /// # Errors
  /// * `JsonError` - If the package.json file could not be read
  pub fn new() -> Result<Project, JsonError> {
    let json = fs::read("package.json")
      .map(|contents| serde_json::from_slice::<Project>(&contents).unwrap());

    if json.is_err() {
      return Err(JsonError::new("Could not read package.json".to_string()));
    }

    Ok(json.unwrap())
  }
}