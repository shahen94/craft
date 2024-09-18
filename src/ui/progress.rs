use std::time::Duration;
use chrono::Local;
use env_logger::{Builder, Logger};
use indicatif::{MultiProgress, ProgressBar};
use indicatif_log_bridge::LogWrapper;
use log::{Level, LevelFilter};
use crate::{
    contracts::{Phase, Progress, ProgressAction, CRAFT_VERBOSE_LOGGING},
    perf::Performance,
};

use super::constants::{COMPLETED, DOWNLOADING, EXTRACTING, LINKING, RESOLVING};

// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct UIProgress {
    #[allow(dead_code)]
    multi_pb: MultiProgress,
    resolving_spinner: ProgressBar,
    downloading_spinner: ProgressBar,
    extracting_spinner: ProgressBar,
    linking_spinner: ProgressBar,

    is_only_verbose: bool,
}

pub fn init_logging() -> Logger {
    use std::io::Write;
    Builder::new()
        .format(|buf, record| {
            let symbol = match record.level() {
                Level::Info => "ℹ️",
                Level::Error => "❌",
                Level::Warn => "⚠️",
                Level::Debug => "🐛",
                Level::Trace => "🔍",
            };
            writeln!(
                buf,
                "{} {} - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                symbol,
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .build()
}

// ─────────────────────────────────────────────────────────────────────────────

impl Default for UIProgress {
    fn default() -> Self {
        let multi_pb = MultiProgress::new();
        let resolving_spinner = multi_pb.add(ProgressBar::new_spinner());
        let downloading_spinner = multi_pb.add(ProgressBar::new_spinner());
        let extracting_spinner = multi_pb.add(ProgressBar::new_spinner());
        let linking_spinner = multi_pb.add(ProgressBar::new_spinner());

        let is_only_verbose = std::env::var(CRAFT_VERBOSE_LOGGING)
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .unwrap_or(false);
        let logger = init_logging();
        let level = logger.filter();
        LogWrapper::new(multi_pb.clone(), logger).try_init().unwrap();
        log::set_max_level(level);



        UIProgress {
            multi_pb,
            resolving_spinner,
            downloading_spinner,
            extracting_spinner,
            linking_spinner,
            is_only_verbose,
        }
    }
}

impl Progress for UIProgress {
    fn set_phase(&self, phase: Phase, took: u128) {
        match phase {
            Phase::Resolving => {
                self.resolving_spinner
                    .set_message(format!("{} Resolving ...", RESOLVING));
                self.resolving_spinner
                    .enable_steady_tick(Duration::from_millis(1));
            }
            Phase::Downloading => {
                self.resolving_spinner.finish();
                self.resolving_spinner
                    .set_message(format!("{} Resolved in {}ms", COMPLETED, took));
                self.downloading_spinner
                    .set_message(format!("{} Downloading ...", DOWNLOADING));
                self.downloading_spinner
                    .enable_steady_tick(Duration::from_millis(1));
            }
            Phase::Extracting => {
                self.downloading_spinner.finish();
                self.downloading_spinner
                    .set_message(format!("{} Downloaded in {}ms", COMPLETED, took));
                self.extracting_spinner
                    .set_message(format!("{} Extracting ...", EXTRACTING));
                self.extracting_spinner
                    .enable_steady_tick(Duration::from_millis(100));
            }
            Phase::Linking => {
                self.extracting_spinner.finish();
                self.extracting_spinner
                    .set_message(format!("{} Extracted in {}ms", COMPLETED, took));
                self.linking_spinner
                    .set_message(format!("{} Linking ...", LINKING));
                self.linking_spinner
                    .enable_steady_tick(Duration::from_millis(100));
            }
        }
    }

    fn finish(&self) {
        self.linking_spinner
            .set_message(format!("{} Linked", COMPLETED));
        self.resolving_spinner.finish();
        self.downloading_spinner.finish();
        self.extracting_spinner.finish();
        self.linking_spinner.finish();
    }

    fn start(&self, rx: std::sync::mpsc::Receiver<ProgressAction>) {
        let mut performance = Performance::default();

        while let Ok(action) = rx.recv() {
            let took = performance.elapsed();
            performance.reset();
            if !self.is_only_verbose {
                self.set_phase(action.phase, took);
            }
        }
        self.finish();
    }
}
