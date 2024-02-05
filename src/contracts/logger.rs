pub trait Logger {
    fn log<S: AsRef<str>>(message: S) -> ();
    fn info<S: AsRef<str>>(message: S) -> ();
    fn warn<S: AsRef<str>>(message: S) -> ();
    fn error<S: AsRef<str>>(message: S) -> ();
}
