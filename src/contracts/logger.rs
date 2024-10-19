pub trait Logger {
    fn info<S: AsRef<str> + std::fmt::Display>(message: S);
    fn warn<S: AsRef<str> + std::fmt::Display>(message: S);
    fn error<S: AsRef<str> + std::fmt::Display>(message: S);
}
