use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar};

use crate::contracts::{Phase, Progress, ProgressAction};

use super::constants::{COMPLETED, DOWNLOADING, EXTRACTING, LINKING, RESOLVING};

// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct UIProgress {
    multi_pb: MultiProgress,
    resolving_spinner: ProgressBar,
    downloading_spinner: ProgressBar,
    extracting_spinner: ProgressBar,
    linking_spinner: ProgressBar,
}

// ─────────────────────────────────────────────────────────────────────────────

impl Progress for UIProgress {
    fn new() -> Self {
        let multi_pb = MultiProgress::new();
        let resolving_spinner = multi_pb.add(ProgressBar::new_spinner());
        let downloading_spinner = multi_pb.add(ProgressBar::new_spinner());
        let extracting_spinner = multi_pb.add(ProgressBar::new_spinner());
        let linking_spinner = multi_pb.add(ProgressBar::new_spinner());

        UIProgress {
            multi_pb,
            resolving_spinner,
            downloading_spinner,
            extracting_spinner,
            linking_spinner,
        }
    }

    fn set_phase(&self, phase: Phase) {
        match phase {
            Phase::Resolving => {
                self.resolving_spinner
                    .set_message(format!("{} Resolving ...", RESOLVING));
                self.resolving_spinner
                    .enable_steady_tick(Duration::from_millis(100));
            }
            Phase::Downloading => {
                self.resolving_spinner.finish();
                self.resolving_spinner.set_message(format!("{} Resolved", COMPLETED));
                self.downloading_spinner
                    .set_message(format!("{} Downloading ...", DOWNLOADING));
                self.downloading_spinner
                    .enable_steady_tick(Duration::from_millis(100));
            }
            Phase::Extracting => {
                self.downloading_spinner.finish();
                self.downloading_spinner.set_message(format!("{} Downloaded", COMPLETED));
                self.extracting_spinner
                    .set_message(format!("{} Extracting ...", EXTRACTING));
                self.extracting_spinner
                    .enable_steady_tick(Duration::from_millis(100));
            }
            Phase::Linking => {
                self.extracting_spinner.finish();
                self.extracting_spinner.set_message(format!("{} Extracted", COMPLETED));
                self.linking_spinner
                    .set_message(format!("{} Linking ...", LINKING));
                self.linking_spinner
                    .enable_steady_tick(Duration::from_millis(100));
            }
        }
    }

    fn finish(&self) {
        self.linking_spinner.set_message(format!("{} Linked", COMPLETED));
        self.resolving_spinner.finish();
        self.downloading_spinner.finish();
        self.extracting_spinner.finish();
        self.linking_spinner.finish();
        // self.multi_pb.clear().unwrap();
    }

    fn start(&self, rx: std::sync::mpsc::Receiver<ProgressAction>) {
        while let Ok(action) = rx.recv() {
          self.set_phase(action.phase);
        }
        self.finish();
    }
}
