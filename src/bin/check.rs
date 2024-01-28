use regex::Regex;
use std::{str::FromStr, string::ToString};

// --------------------------------------------
// Version
// --------------------------------------------
trait Version: ToString {
    fn new(version: &str) -> Self;

    fn satisfies(&self, version: &str) -> bool;
}

// --------------------------------------------
// Package
// --------------------------------------------
#[derive(Debug)]
struct Package<T: Version> {
    pub name: String,
    pub version: T,
}

// --------------------------------------------
// VersionPart
// --------------------------------------------
#[derive(Debug)]
struct VersionConstraint {
    pub operator: Operator,
    pub major: VersionField,
    pub minor: VersionField,
    pub patch: VersionField,
    pub pre_release: Option<String>,
    pub build: Option<String>,
}

impl ToString for VersionConstraint {
    fn to_string(&self) -> String {
        let mut version = format!(
            "{}.{}.{}",
            self.major.to_string(),
            self.minor.to_string(),
            self.patch.to_string()
        );

        match &self.operator {
            Operator::Equal => {}
            operator => version = format!("{}{}", operator.to_string(), version),
        }

        if let Some(pre_release) = &self.pre_release {
            version = format!("{}-{}", version, pre_release);
        }

        if let Some(build) = &self.build {
            version = format!("{}+{}", version, build);
        }

        version
    }
}

// --------------------------------------------
// Operator
// --------------------------------------------
#[derive(Debug)]
enum Operator {
    Tilde,
    Caret,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "~" => Ok(Operator::Tilde),
            "^" => Ok(Operator::Caret),
            ">" => Ok(Operator::GreaterThan),
            ">=" => Ok(Operator::GreaterThanOrEqual),
            "<" => Ok(Operator::LessThan),
            "<=" => Ok(Operator::LessThanOrEqual),
            "=" => Ok(Operator::Equal),
            _ => Err(()),
        }
    }
}

impl ToString for Operator {
    fn to_string(&self) -> String {
        match self {
            Operator::Tilde => "~".to_string(),
            Operator::Caret => "^".to_string(),
            Operator::GreaterThan => ">".to_string(),
            Operator::GreaterThanOrEqual => ">=".to_string(),
            Operator::LessThan => "<".to_string(),
            Operator::LessThanOrEqual => "<=".to_string(),
            Operator::Equal => "=".to_string(),
        }
    }
}

// --------------------------------------------
// VersionField
// --------------------------------------------
#[derive(Debug)]
enum VersionField {
    Exact(u64),
    Wildcard,
}

impl ToString for VersionField {
    fn to_string(&self) -> String {
        match self {
            VersionField::Exact(value) => format!("{}", value),
            VersionField::Wildcard => "*".to_string(),
        }
    }
}

// --------------------------------------------
// VersionImpl
// --------------------------------------------
#[derive(Debug)]
struct VersionImpl {
    inner: Vec<VersionConstraint>,
}

impl ToString for VersionImpl {
    fn to_string(&self) -> String {
        self.inner
            .iter()
            .map(|constraint| constraint.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl VersionImpl {
    fn parse_constraints(version: &str) -> Vec<VersionConstraint> {
        let mut constraints = vec![];

        if version.contains(">") || version.contains("<") {
            return Self::parse_range(&version);
        }

        constraints.push(Self::parse_single_constraint(&version));

        constraints
    }

    fn parse_single_constraint(version: &str) -> VersionConstraint {
        let mut major = VersionField::Wildcard;
        let mut minor = VersionField::Wildcard;
        let mut patch = VersionField::Wildcard;
        let mut pre_release = None;
        let mut build = None;
        let mut operator = Operator::Equal;

        if version == "*" || version == "x" || version == "latest" {
            return VersionConstraint {
                operator,
                major,
                minor,
                patch,
                pre_release,
                build,
            };
        }

        let semver_regex = r"^(?P<operator>\^|~|=)?(?P<major>\d+|x|\*)(?:\.(?P<minor>\d+|x|\*))?(?:\.(?P<patch>\d+|x|\*))?(?:[-.](?P<alpha>[a-zA-Z0-9-]+))?(?:\+(?P<build>[a-zA-Z0-9-]+))?$";
        let semver_regex = regex::Regex::new(semver_regex).unwrap();

        let captures = semver_regex
            .captures(&version)
            .expect(format!("Invalid version: {}", version).as_str());

        if let Some(symbol_value) = captures.name("operator") {
            operator = symbol_value.as_str().parse::<Operator>().unwrap();
        }
        if let Some(major_value) = captures.name("major") {
            if major_value.as_str() != "*" && major_value.as_str() != "x" {
                major = VersionField::Exact(major_value.as_str().parse::<u64>().unwrap());
            }
        }

        if let Some(minor_value) = captures.name("minor") {
            if minor_value.as_str() != "*" && minor_value.as_str() != "x" {
                minor = VersionField::Exact(minor_value.as_str().parse::<u64>().unwrap());
            }
        }

        if let Some(patch_value) = captures.name("patch") {
            if patch_value.as_str() != "*" && patch_value.as_str() != "x" {
                patch = VersionField::Exact(patch_value.as_str().parse::<u64>().unwrap());
            }
        }

        if let Some(alpha_value) = captures.name("alpha") {
            pre_release = Some(alpha_value.as_str().to_string());
        }

        if let Some(build_value) = captures.name("build") {
            build = Some(build_value.as_str().to_string());
        }

        VersionConstraint {
            operator,
            major,
            minor,
            patch,
            pre_release,
            build,
        }
    }

    fn parse_range(version: &str) -> Vec<VersionConstraint> {
        let regex = r"^(?P<start_operator>[<>]=?|~|\^)?(?P<start_major>\d+|x|\*)(?:\.(?P<start_minor>\d+|x|\*))?(?:\.(?P<start_patch>\d+|x|\*))?(?:(?P<connector>,)?\s*(?P<end_operator>[<>]=?|~|\^)?(?P<end_major>\d+|x|\*)(?:\.(?P<end_minor>\d+|x|\*))?(?:\.(?P<end_patch>\d+|x|\*))?)?$";
        
        let mut start_operator = Operator::Equal;
        let mut start_major = VersionField::Wildcard;
        let mut start_minor = VersionField::Wildcard;
        let mut start_patch = VersionField::Wildcard;
        let mut connector = None;
        let mut end_operator = Operator::Equal;
        let mut end_major = VersionField::Wildcard;
        let mut end_minor = VersionField::Wildcard;
        let mut end_patch = VersionField::Wildcard;

        let regex = Regex::new(regex).unwrap();

        let captures = regex
            .captures(&version)
            .expect(format!("Invalid version: {}", version).as_str());

        if let Some(start_operator_value) = captures.name("start_operator") {
            start_operator = start_operator_value.as_str().parse::<Operator>().unwrap();
        }

        if let Some(start_major_value) = captures.name("start_major") {
            if start_major_value.as_str() != "*" && start_major_value.as_str() != "x" {
                start_major = VersionField::Exact(start_major_value.as_str().parse::<u64>().unwrap());
            }
        }

        if let Some(start_minor_value) = captures.name("start_minor") {
            if start_minor_value.as_str() != "*" && start_minor_value.as_str() != "x" {
                start_minor = VersionField::Exact(start_minor_value.as_str().parse::<u64>().unwrap());
            }
        }

        if let Some(start_patch_value) = captures.name("start_patch") {
            if start_patch_value.as_str() != "*" && start_patch_value.as_str() != "x" {
                start_patch = VersionField::Exact(start_patch_value.as_str().parse::<u64>().unwrap());
            }
        }

        if let Some(connector_value) = captures.name("connector") {
            connector = Some(connector_value.as_str().to_string());
        }

        if let Some(end_operator_value) = captures.name("end_operator") {
            end_operator = end_operator_value.as_str().parse::<Operator>().unwrap();
        }

        if let Some(end_major_value) = captures.name("end_major") {
            if end_major_value.as_str() != "*" && end_major_value.as_str() != "x" {
                end_major = VersionField::Exact(end_major_value.as_str().parse::<u64>().unwrap());
            }
        }

        if let Some(end_minor_value) = captures.name("end_minor") {
            if end_minor_value.as_str() != "*" && end_minor_value.as_str() != "x" {
                end_minor = VersionField::Exact(end_minor_value.as_str().parse::<u64>().unwrap());
            }
        }

        if let Some(end_patch_value) = captures.name("end_patch") {
            if end_patch_value.as_str() != "*" && end_patch_value.as_str() != "x" {
                end_patch = VersionField::Exact(end_patch_value.as_str().parse::<u64>().unwrap());
            }
        }

        let mut constraints = vec![];

        constraints.push(VersionConstraint {
            operator: start_operator,
            major: start_major,
            minor: start_minor,
            patch: start_patch,
            pre_release: None,
            build: None,
        });

        constraints.push(VersionConstraint {
          operator: end_operator,
          major: end_major,
          minor: end_minor,
          patch: end_patch,
          pre_release: None,
          build: None,
      });

        constraints
    }
}

impl Version for VersionImpl {
    fn new(version: &str) -> Self {
        let mut inner = Self::parse_constraints(&version);

        Self { inner }
    }

    fn satisfies(&self, version: &str) -> bool {
        todo!()
    }
}

impl Package<VersionImpl> {
    pub fn new(package_name: &str) -> Self {
        let parts = package_name.rsplitn(2, '@').collect::<Vec<_>>();

        match parts.len() {
            1 => Self {
                name: parts[0].to_string(),
                version: VersionImpl::new("*"),
            },
            2 => {
                let escaped_version = if parts[0] == "latest" {
                    "*".to_string()
                } else {
                    parts[0].to_string()
                };

                return Self {
                    name: if package_name.starts_with("@") {
                        format!("{}", parts[1])
                    } else {
                        parts[1].to_string()
                    },

                    version: VersionImpl::new(&escaped_version),
                };
            }
            _ => panic!("Invalid package name: {}", package_name),
        }
    }
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
        let package = Package::new(&version);
        println!("{:?}", package.version.inner.len());
    }
}