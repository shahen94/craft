use std::{sync::Arc, time::Duration};

use console::Emoji;
use indicatif::{MultiProgress, ProgressBar};

static RESOLVING: Emoji<'_, '_> = Emoji("🔍  ", "");
static DOWNLOADING: Emoji<'_, '_> = Emoji("🚚  ", "");
static LINKING: Emoji<'_, '_> = Emoji("🔗  ", "");
static EXTRACTING: Emoji<'_, '_> = Emoji("📦  ", "");

#[derive(Debug)]
enum Phase {
    Resolving,
    Downloading,
    Extracting,
    Linking,
}

struct ProgressAction {
  phase: Phase,
  progress: u64,
  size: u64,
}

trait Progress {
    fn new() -> Self;
    fn start(&self, rx: std::sync::mpsc::Receiver<ProgressAction>);
    fn set_phase(&self, phase: Phase);
    fn set_progress(&self, progress: u64);
    fn set_size(&self, size: u64);
    fn increment(&self);
    fn finish(&self);
}

fn create_progress_bar<T>() -> T where T: Progress {
  T::new()
}

struct UIProgressImpl {
    multi_pb: MultiProgress,
    resolving_spinner: ProgressBar,
    downloading_spinner: ProgressBar,
    extracting_spinner: ProgressBar,
    linking_spinner: ProgressBar,
    bottom_pb: ProgressBar,
}

impl Progress for UIProgressImpl {
    fn new() -> Self {
        let multi_pb = MultiProgress::new();
        let resolving_spinner = multi_pb.add(ProgressBar::new_spinner());
        let downloading_spinner = multi_pb.add(ProgressBar::new_spinner());
        let extracting_spinner = multi_pb.add(ProgressBar::new_spinner());
        let linking_spinner = multi_pb.add(ProgressBar::new_spinner());
        let bottom_pb = multi_pb.add(ProgressBar::new(4));

        UIProgressImpl {
            multi_pb,
            resolving_spinner,
            downloading_spinner,
            extracting_spinner,
            linking_spinner,
            bottom_pb,
        }
    }

    fn set_phase(&self, phase: Phase) {
        match phase {
            Phase::Resolving => {
                self.resolving_spinner.set_message(format!("{} Resolving ...", RESOLVING));
                self.resolving_spinner.enable_steady_tick(Duration::from_millis(100));
            }
            Phase::Downloading => {
                self.resolving_spinner.finish();
                self.downloading_spinner.set_message(format!("{} Downloading ...", DOWNLOADING));
                self.downloading_spinner.enable_steady_tick(Duration::from_millis(100));
            }
            Phase::Extracting => {
                self.downloading_spinner.finish();
                self.extracting_spinner.set_message(format!("{} Extracting ...", EXTRACTING));
                self.extracting_spinner.enable_steady_tick(Duration::from_millis(100));
            }
            Phase::Linking => {
                self.extracting_spinner.finish();
                self.linking_spinner.set_message(format!("{} Linking ...", LINKING));
                self.linking_spinner.enable_steady_tick(Duration::from_millis(100));
            }
        }
    }

    fn finish(&self) {
        self.resolving_spinner.finish();
        self.downloading_spinner.finish();
        self.extracting_spinner.finish();
        self.linking_spinner.finish();
        self.multi_pb.clear().unwrap();
    }

    fn set_progress(&self, progress: u64) {
        self.bottom_pb.set_position(progress);
    }

    fn increment(&self) {
        self.bottom_pb.inc(1);
    }

    fn start(&self, rx: std::sync::mpsc::Receiver<ProgressAction>) {
        for action in rx.iter() {
            self.set_phase(action.phase);
            self.set_size(action.size);
            self.set_progress(action.progress);
        }
    }

    fn set_size(&self, size: u64) {
        self.bottom_pb.set_length(size);
    }
}


fn main() {
    let mut progress = Arc::new(create_progress_bar::<UIProgressImpl>());

    progress.set_phase(Phase::Resolving);
    progress.set_size(100);
    let (tx, rx) = std::sync::mpsc::channel::<ProgressAction>();

    let p_m = progress.clone();
    let ui_thread = std::thread::spawn(move || {
      p_m.start(rx);
    });

    let process_thread = std::thread::spawn(move || {
        for i in 0..4 {
            let phase = if i == 0 {
                Phase::Resolving
            } else if i == 1 {
                Phase::Downloading
            } else if i == 2 {
                Phase::Extracting
            } else {
                Phase::Linking
            };

            let action = ProgressAction {
                phase,
                progress: i,
                size: 4,
            };
            tx.send(action).unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    progress.set_phase(Phase::Downloading);

    // wait for threads

    process_thread.join().unwrap();
    ui_thread.join().unwrap();
    progress.finish();
}