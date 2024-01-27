#[derive(Debug, Clone)]
pub enum VersionField {
  Exact(u64),
  Wildcard,
}


#[derive(Debug, Clone)]
pub struct VersionInfo {
  major: VersionField,
  minor: VersionField,
  patch: VersionField,
  pre: Option<String>,
}

impl ToString for VersionInfo {
  fn to_string(&self) -> String {
    let major = match self.major {
      VersionField::Exact(major) => major.to_string(),
      VersionField::Wildcard => "x".to_string(),
    };
    let minor = match self.minor {
      VersionField::Exact(minor) => minor.to_string(),
      VersionField::Wildcard => "x".to_string(),
    };
    let patch = match self.patch {
      VersionField::Exact(patch) => patch.to_string(),
      VersionField::Wildcard => "x".to_string(),
    };
    let pre = match self.pre {
      Some(ref pre) => format!("-{}", pre),
      None => "".to_string(),
    };

    format!("{}.{}.{}{}", major, minor, patch, pre)
  }
}

impl VersionInfo {
  fn parse_version_field(field: &str) -> VersionField {
    if field.contains("||") {
      return VersionField::Exact(field.split("||").last().unwrap().parse::<u64>().unwrap());
    }
    match field {
      "*" | "x" => VersionField::Wildcard,
      _ => VersionField::Exact(field.parse::<u64>().expect(
        &format!("Invalid version field: {}", field)
      )),
    }
  }

  pub fn parse(version: &str) -> (VersionField, VersionField, VersionField, Option<String>) {
    if version == "latest" || version == "*" {
      // Wildcards
      return (VersionField::Wildcard, VersionField::Wildcard, VersionField::Wildcard, None);
    }

    let mut parts = version.splitn(3, '.').collect::<Vec<_>>();

    let mut major = VersionField::Wildcard;
    let mut minor = VersionField::Wildcard;
    let mut patch = VersionField::Wildcard;
    let mut pre: Option<String> = None;
    
    match parts.len() {
      1 => {
        major = Self::parse_version_field(parts[0]);
      }
      2 => {
        major = Self::parse_version_field(parts[0]);
        minor = Self::parse_version_field(parts[1]);
      }
      3 => {
        major = Self::parse_version_field(parts[0]);
        minor = Self::parse_version_field(parts[1]);
        let mut parts = parts[2].splitn(2, '-').collect::<Vec<_>>();
        patch = Self::parse_version_field(parts[0]);
        if parts.len() == 2 {
          pre = Some(parts[1].to_string());
        }
      }
      _ => panic!("Invalid version: {}", version),      
    }

    return (major, minor, patch, pre)
  }

  pub fn new(version: &str) -> Self {
    let (major, minor, patch, pre) = Self::parse(version);

    Self {
      major,
      minor,
      patch,
      pre,
    }
  }

  pub fn satisfies(&self, version: &str) -> bool {
    let version = Self::parse(version);

    let is_major_satisfied = match (&self.major, &version.0) {
      (VersionField::Exact(major), VersionField::Exact(v)) => major == v,
      (VersionField::Wildcard, _) => true,
      _ => false,
    };

    let is_minor_satisfied = match (&self.minor, &version.1) {
      (VersionField::Exact(minor), VersionField::Exact(v)) => minor == v,
      (VersionField::Wildcard, _) => true,
      _ => false,
    };

    let is_patch_satisfied = match (&self.patch, &version.2) {
      (VersionField::Exact(patch), VersionField::Exact(v)) => patch == v,
      (VersionField::Wildcard, _) => true,
      _ => false,
    };

    let is_pre_satisfied = match (&self.pre, &version.3) {
      (Some(pre), Some(v)) => pre == v,
      (None, _) => true,
      _ => false,
    };
    
    is_major_satisfied && is_minor_satisfied && is_patch_satisfied && is_pre_satisfied
  }

  pub fn is_exact(&self) -> bool {
    match (&self.major, &self.minor, &self.patch) {
      (VersionField::Exact(_), VersionField::Exact(_), VersionField::Exact(_)) => true,
      _ => false,
    }
  }
}
