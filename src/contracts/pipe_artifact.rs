pub trait PipeArtifact<T> {
    fn get_artifacts(&self) -> T;
}

pub trait PipeExtras<T> {
    fn get_extras(&self) -> T;
}
