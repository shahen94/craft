/// Package version
/// 
/// # Example
/// ```
/// let version = PackageVersion::new(1, 0, 0);
/// ```
#[derive(Debug)]
pub struct PackageVersion {
  pub major: u64,
  pub minor: u64,
  pub patch: u64,
}

impl PackageVersion {
  pub fn new(major: u64, minor: u64, patch: u64) -> Self {
    Self {
      major,
      minor,
      patch,
    }
  }
}