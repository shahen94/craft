pub trait Logger {
  fn log<S: AsRef<str>>(&self, message: S) -> ();
  fn info<S: AsRef<str>>(&self, message: S) -> ();
  fn debug<S: AsRef<str>>(&self, message: S) -> ();
  fn warn<S: AsRef<str>>(&self, message: S) -> ();
  fn error<S: AsRef<str>>(&self, message: S) -> ();
}