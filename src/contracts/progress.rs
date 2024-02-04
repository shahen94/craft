use std::sync::mpsc::Receiver;

#[derive(Debug)]
pub enum Phase {
    Resolving,
    Downloading,
    Extracting,
    Linking,
}

#[derive(Debug)]
pub struct ProgressAction {
    pub phase: Phase,
}

impl ProgressAction {
    pub fn new(phase: Phase) -> Self {
        Self {
            phase,
        }
    }
}

pub trait Progress {
    fn new() -> Self;
    fn start(&self, rx: Receiver<ProgressAction>);
    fn set_phase(&self, phase: Phase);
    fn finish(&self);
}
