pub struct GitPackage {
  pub url: String,
}

impl GitPackage {
  pub fn new(url: &str) -> Self {
    GitPackage { url: url.to_string() }
  }
}