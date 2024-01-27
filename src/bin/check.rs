
trait Version {
  fn parse(version: &str) -> Self;

  fn satisfies(&self, version: &str) -> bool;
}

#[derive(Debug)]
struct Package<T: Version> {
  name: String,
  version: T,
  raw_version: String,
}

#[derive(Debug)]
enum VersionImpl {
  Exact(VersionPart),
  Wildcard,
  LessThan(VersionPart),
  GreaterThan(VersionPart),
  LessThanEqual(VersionPart),
  GreaterThanEqual(VersionPart),
  Tilde(VersionPart),
  Caret(VersionPart),
  Between(VersionPart, VersionPart),
}

#[derive(Debug)]
struct VersionPart {
  major: VersionField,
  minor: VersionField,
  patch: VersionField,
  pre_release: Option<String>,
}

#[derive(Debug)]
enum VersionField {
  Exact(String),
  Wildcard,
}


fn main() {
  let versions = vec![
    "react@16.13.1",
    "@nest/core@16.13.1",
    "react@^16.13.1",
    "react@~16.13.1",
    "react@16.13",
    "react@16",
    "react@^16",
    "react@~16",
    "react@16.x",
    "react@^16.*",
    "react@~16.*",
    "react@*",
    "react@latest",
    "react@>=10.0.0 <11.0.0",
    "react@>=10.0.0,<11.0.0",
    "react@>=10.0.0",
    "react@<10.0.0",
    "react@10.0.0||11||13",
  ];

  // let package = Package::new(package_name)
  // package.version.satisfies(version) -> true | false;

  for version in versions {
    // println!("{}, {:?}", package.raw_version, package.version);
  }
}