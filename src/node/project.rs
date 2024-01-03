use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;
use tokio::fs;

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
    pub dev_dependencies: HashMap<String, String>,

    pub version: Option<String>,
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
    pub async fn new(source: Option<PathBuf>) -> Result<Project, JsonError> {
        let source = match source {
            Some(source) => source,
            None => PathBuf::from("package.json"),
        };
        let json = fs::read(source)
            .await
            .map(|contents| serde_json::from_slice::<Project>(&contents).unwrap())?;


        Ok(json)
    }
}
