use crate::package::NpmPackage;

pub trait PipeResolveArtifact {
  fn get_artifacts(&self) -> Vec<NpmPackage>;
}