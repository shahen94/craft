pub trait PipeArtifact<T> {
    fn get_artifacts(&self) -> T;
}
