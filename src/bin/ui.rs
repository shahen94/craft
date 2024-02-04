use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};


fn main() {
  // Create a multi-progress bar to manage multiple progress bars
  let multi_pb = MultiProgress::new();

  // Create a spinner for each step
  let resolving_spinner = multi_pb.add(ProgressBar::new_spinner());
  let downloading_spinner = multi_pb.add(ProgressBar::new_spinner());
  let extracting_spinner = multi_pb.add(ProgressBar::new_spinner());
  let linking_spinner = multi_pb.add(ProgressBar::new_spinner());

  // Configure spinners
  configure_spinner(&resolving_spinner, "Resolving ...");
  configure_spinner(&downloading_spinner, "Downloading ...");
  configure_spinner(&extracting_spinner, "Extracting ...");
  configure_spinner(&linking_spinner, "Linking ...");

  // Create a bottom progress bar
  let bottom_pb = multi_pb.add(ProgressBar::new(4));

  bottom_pb.set_style(
      ProgressStyle::default_bar()
          .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {per_sec} ETA {eta}")
          .unwrap()
          .progress_chars("##-")
  );

  // Start the animation loop
  for _ in 0..4 {
      // Increment the bottom progress bar
      bottom_pb.inc(1);
      std::thread::sleep(std::time::Duration::from_secs(1));
  }

  // Finish the progress bars
  resolving_spinner.finish();
  downloading_spinner.finish();
  extracting_spinner.finish();
  linking_spinner.finish();
  multi_pb.clear().unwrap();
}

fn configure_spinner(spinner: &ProgressBar, message: &str) {
  spinner.set_style(
      ProgressStyle::default_spinner()
          .tick_strings(&["⠈", "⠐", "⠘", "⠰"])
          .template("{spinner} {wide_msg}")
          .unwrap()
  );
  let msg = format!("{} ...", message);
  spinner.set_message(msg);
  spinner.enable_steady_tick(Duration::from_millis(100));
}
