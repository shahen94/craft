use regex::Regex;
use std::{str::FromStr, string::ToString};

const SEMVER_REGEX: &str = r"^(?P<operator>\^|~|=)?(?P<major>\d+|x|\*)(?:\.(?P<minor>\d+|x|\*))?(?:\.(?P<patch>\d+|x|\*))?(?:[-.](?P<alpha>[a-zA-Z0-9-]+))?(?:\+(?P<build>[a-zA-Z0-9-]+))?$";

// --------------------------------------------
// Version
// --------------------------------------------
trait Version: ToString {
    fn new(version: &str) -> Self;

    fn satisfies(&self, version: &str) -> bool;
}

trait Satisfies {
    fn satisfies(&self, version: &str) -> bool;
}

// --------------------------------------------
// Package
// --------------------------------------------
#[derive(Debug)]
struct Package<T: Version> {
    pub name: String,
    pub version: T,
    pub raw_version: String,
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

impl VersionConstraint {
    pub fn parse(version: &str) -> Self {
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

        let semver_regex = regex::Regex::new(SEMVER_REGEX).unwrap();

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
}

impl Satisfies for VersionConstraint {
    fn satisfies(&self, version: &str) -> bool {
        let version = VersionConstraint::parse(version);

        if !self.major.satisfies(&version.major.to_string()) {
            return false;
        }

        if !self.minor.satisfies(&version.minor.to_string()) {
            return false;
        }

        if !self.patch.satisfies(&version.patch.to_string()) {
            return false;
        }

        if let Some(pre_release) = &self.pre_release {
            if let Some(version_pre_release) = &version.pre_release {
                if pre_release != version_pre_release {
                    return false;
                }
            }
            return false;
        }

        if let Some(build) = &self.build {
            if let Some(version_build) = &version.build {
                if build != version_build {
                    return false;
                }
            }
            return false;
        }

        true
    }
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
#[derive(Debug, PartialEq)]
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

impl Satisfies for VersionField {
    fn satisfies(&self, version: &str) -> bool {
        match self {
            VersionField::Exact(value) => {
                if version == "*" || version == "x" || version == "latest" {
                    return true;
                }

                let version = VersionConstraint::parse(version);

                match version.major {
                    VersionField::Exact(major) => {
                        if major != *value {
                            return false;
                        }
                    }
                    _ => {}
                }

                match version.minor {
                    VersionField::Exact(minor) => {
                        if minor != *value {
                            return false;
                        }
                    }
                    _ => {}
                }

                match version.patch {
                    VersionField::Exact(patch) => {
                        if patch != *value {
                            return false;
                        }
                    }
                    _ => {}
                }

                true
            }
            VersionField::Wildcard => true,
        }
    }
}

// --------------------------------------------
// Connector
// --------------------------------------------
#[derive(Debug, PartialEq)]
enum Connector {
    And,
    Or,
}

impl FromStr for Connector {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "," | " " => Ok(Connector::And),
            "|" | "||" => Ok(Connector::Or),
            _ => Ok(Connector::And),
        }
    }
}

// --------------------------------------------
// VersionGroup
// --------------------------------------------
#[derive(Debug)]
struct VersionGroup {
    constraints: Vec<VersionConstraint>,
    connector: Connector,
}

impl VersionGroup {
    pub fn new(constraints: Vec<VersionConstraint>, connector: Connector) -> Self {
        Self {
            constraints,
            connector,
        }
    }
}

impl ToString for VersionGroup {
    fn to_string(&self) -> String {
        let mut constraints = self
            .constraints
            .iter()
            .map(|constraint| constraint.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        if self.connector == Connector::Or {
            constraints = format!("({})", constraints);
        }

        constraints
    }
}

// --------------------------------------------
// VersionImpl
// --------------------------------------------
#[derive(Debug)]
struct VersionImpl {
    inner: Vec<VersionGroup>,
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
    fn parse_constraints(version: &str) -> Vec<VersionGroup> {
        if version.contains("||") {
            let parts = version.split("||").collect::<Vec<_>>();
            let mut groups = vec![];

            for part in parts {
                let mut constraints = vec![];
                if part.contains(">") || part.contains("<") {
                    constraints.append(&mut Self::parse_range(&part));
                } else {
                    constraints.push(VersionConstraint::parse(part));
                }

                let group = VersionGroup::new(constraints, Connector::Or);
                groups.push(group);
            }

            return groups;
        }

        if version.contains(">") || version.contains("<") {
            let constraints = Self::parse_range(&version);

            return vec![VersionGroup::new(constraints, Connector::And)];
        }

        let constraint = VersionConstraint::parse(version);

        vec![VersionGroup::new(vec![constraint], Connector::And)]
    }

    fn parse_range(version: &str) -> Vec<VersionConstraint> {
        let regex = r"^(?P<start_operator>[<>]=?|~|\^)?(?P<start_major>\d+|x|\*)(?:\.(?P<start_minor>\d+|x|\*))?(?:\.(?P<start_patch>\d+|x|\*))?(?:(?P<connector>,|\|\|)?\s*(?P<end_operator>[<>]=?|~|\^)?(?P<end_major>\d+|x|\*)(?:\.(?P<end_minor>\d+|x|\*))?(?:\.(?P<end_patch>\d+|x|\*))?)?$";

        let mut start_operator = Operator::Equal;
        let mut start_major = VersionField::Wildcard;
        let mut start_minor = VersionField::Wildcard;
        let mut start_patch = VersionField::Wildcard;

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
                start_major =
                    VersionField::Exact(start_major_value.as_str().parse::<u64>().unwrap());
            }
        }

        if let Some(start_minor_value) = captures.name("start_minor") {
            if start_minor_value.as_str() != "*" && start_minor_value.as_str() != "x" {
                start_minor =
                    VersionField::Exact(start_minor_value.as_str().parse::<u64>().unwrap());
            }
        }

        if let Some(start_patch_value) = captures.name("start_patch") {
            if start_patch_value.as_str() != "*" && start_patch_value.as_str() != "x" {
                start_patch =
                    VersionField::Exact(start_patch_value.as_str().parse::<u64>().unwrap());
            }
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

        // Check if there was any start version specified

        if start_major != VersionField::Wildcard
            || start_minor != VersionField::Wildcard
            || start_patch != VersionField::Wildcard
        {
            constraints.push(VersionConstraint {
                operator: start_operator,
                major: start_major,
                minor: start_minor,
                patch: start_patch,
                pre_release: None,
                build: None,
            });
        }

        // Check if there was any end version specified

        if end_major != VersionField::Wildcard
            || end_minor != VersionField::Wildcard
            || end_patch != VersionField::Wildcard
        {
            constraints.push(VersionConstraint {
                operator: end_operator,
                major: end_major,
                minor: end_minor,
                patch: end_patch,
                pre_release: None,
                build: None,
            });
        }

        constraints
    }
}

impl Version for VersionImpl {
    fn new(version: &str) -> Self {
        let inner = Self::parse_constraints(&version);

        Self { inner }
    }

    fn satisfies(&self, version: &str) -> bool {
        true
    }
}

impl Package<VersionImpl> {
    pub fn new(package_name: &str) -> Self {
        let parts = package_name.rsplitn(2, '@').collect::<Vec<_>>();

        match parts.len() {
            1 => Self {
                name: parts[0].to_string(),
                version: VersionImpl::new("*"),
                raw_version: "*".to_string(),
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
                    raw_version: escaped_version,
                };
            }
            _ => panic!("Invalid package name: {}", package_name),
        }
    }
}

fn main() {
    struct Checks {
        pub name: String,
        pub assertions: Vec<String>,
    }
    let checks = vec![
        Checks {
            name: "react@16.13.1".to_string(),
            assertions: vec![
                "16.13.1".to_string(),
                "16.13".to_string(),
                "16".to_string(),
                "16.x".to_string(),
                "16.*".to_string(),
                "*".to_string(),
                "latest".to_string(),
            ],
        },
        Checks {
            name: "@nest/core@16.13.1".to_string(),
            assertions: vec![
                "16.13.1".to_string(),
                "16.13".to_string(),
                "16".to_string(),
                "16.x".to_string(),
                "16.*".to_string(),
                "*".to_string(),
                "latest".to_string(),
            ],
        },
        Checks {
            name: "react@^16.13.1".to_string(),
            assertions: vec![
                "16.13.1".to_string(),
                "16.13".to_string(),
                "16".to_string(),
                "16.x".to_string(),
                "16.*".to_string(),
                "*".to_string(),
                "latest".to_string(),
            ],
        },
        Checks {
            name: "react@~16.13.1".to_string(),
            assertions: vec![
                "16.13.1".to_string(),
                "16.13".to_string(),
                "16".to_string(),
                "16.x".to_string(),
                "16.*".to_string(),
                "*".to_string(),
                "latest".to_string(),
            ],
        },
        Checks {
            name: "react@16.13".to_string(),
            assertions: vec![
                "16.13.1".to_string(),
                "16.13".to_string(),
                "16".to_string(),
                "16.x".to_string(),
                "16.*".to_string(),
                "*".to_string(),
                "latest".to_string(),
            ],
        },
        Checks {
            name: "react@16".to_string(),
            assertions: vec![
                "16.13.1".to_string(),
                "16.13".to_string(),
                "16".to_string(),
                "16.x".to_string(),
                "16.*".to_string(),
                "*".to_string(),
                "latest".to_string(),
            ],
        },
        Checks {
            name: "react@^16".to_string(),
            assertions: vec![
                "16.13.1".to_string(),
                "16.13".to_string(),
                "16".to_string(),
                "16.x".to_string(),
                "16.*".to_string(),
                "*".to_string(),
                "latest".to_string(),
            ],
        },
        Checks {
            name: "react@~16".to_string(),
            assertions: vec![
                "16.13.1".to_string(),
                "16.13".to_string(),
                "16".to_string(),
                "16.x".to_string(),
                "16.*".to_string(),
                "*".to_string(),
                "latest".to_string(),
            ],
        },
        Checks {
            name: "react@16.x".to_string(),
            assertions: vec![
                "16.13.1".to_string(),
                "16.13".to_string(),
                "16".to_string(),
                "16.x".to_string(),
                "16.*".to_string(),
                "*".to_string(),
                "latest".to_string(),
            ],
        },
        Checks {
            name: "react@^16.*".to_string(),
            assertions: vec![
                "16.13.1".to_string(),
                "16.13".to_string(),
                "16.14".to_string(),
                "16.15".to_string(),
                "16.16".to_string(),
                "16.17".to_string(),
                "16.18".to_string(),
                "16.19".to_string(),
                "16.20".to_string(),
                "16.21".to_string(),
                "16.22".to_string(),
                "16.23".to_string(),
                "16.24".to_string(),
                "16.25".to_string(),
                "16.26".to_string(),
                "16.27".to_string(),
                "16.28".to_string(),
                "16.29".to_string(),
                "16.30".to_string(),
                "16.31".to_string(),
                "16.32".to_string(),
                "16.33".to_string(),
                "16.34".to_string(),
                "16.35".to_string(),
                "16.36".to_string(),
                "16.37".to_string(),
                "16.38".to_string(),
                "16.39".to_string(),
                "16.40".to_string(),
                "16.41".to_string(),
                "16.42".to_string(),
                "16.43".to_string(),
                "16.44".to_string(),
                "16.45".to_string(),
                "16.46".to_string(),
                "16.47".to_string(),
                "16.48".to_string(),
                "16.49".to_string(),
                "16.50".to_string(),
                "16.51".to_string(),
                "16.52".to_string(),
                "16.53".to_string(),
                "16.54".to_string(),
                "16.55".to_string(),
                "16.56".to_string(),
                "16.57".to_string(),
                "16.58".to_string(),
            ],
        },
        Checks {
            name: "react@~16.*".to_string(),
            assertions: vec![
                "16.13.1".to_string(),
                "16.13".to_string(),
                "16.14".to_string(),
                "16.15".to_string(),
                "16.16".to_string(),
                "16.17".to_string(),
                "16.18".to_string(),
                "16.19".to_string(),
                "16.20".to_string(),
                "16.21".to_string(),
                "16.22".to_string(),
                "16.23".to_string(),
                "16.24".to_string(),
                "16.25".to_string(),
                "16.26".to_string(),
                "16.27".to_string(),
                "16.28".to_string(),
                "16.29".to_string(),
                "16.30".to_string(),
                "16.31".to_string(),
                "16.32".to_string(),
                "16.33".to_string(),
                "16.34".to_string(),
                "16.35".to_string(),
                "16.36".to_string(),
                "16.37".to_string(),
                "16.38".to_string(),
                "16.39".to_string(),
                "16.40".to_string(),
                "16.41".to_string(),
                "16.42".to_string(),
                "16.43".to_string(),
                "16.44".to_string(),
                "16.45".to_string(),
                "16.46".to_string(),
                "16.47".to_string(),
                "16.48".to_string(),
                "16.49".to_string(),
                "16.50".to_string(),
                "16.51".to_string(),
                "16.52".to_string(),
                "16.53".to_string(),
                "16.54".to_string(),
                "16.55".to_string(),
                "16.56".to_string(),
                "16.57".to_string(),
                "16.58".to_string(),
            ],
        },
        Checks {
            name: "react@*".to_string(),
            assertions: vec![
                "16.13.1".to_string(),
                "16.13".to_string(),
                "16.14".to_string(),
                "16.15".to_string(),
                "16.16".to_string(),
                "16.17".to_string(),
                "16.18".to_string(),
                "16.19".to_string(),
                "16.20".to_string(),
                "16.21".to_string(),
                "16.22".to_string(),
                "16.23".to_string(),
                "16.24".to_string(),
                "16.25".to_string(),
                "16.26".to_string(),
                "16.27".to_string(),
                "16.28".to_string(),
                "16.29".to_string(),
                "16.30".to_string(),
                "16.31".to_string(),
                "16.32".to_string(),
                "16.33".to_string(),
                "16.34".to_string(),
                "16.35".to_string(),
                "16.36".to_string(),
                "16.37".to_string(),
                "16.38".to_string(),
                "16.39".to_string(),
                "16.40".to_string(),
                "16.41".to_string(),
                "16.42".to_string(),
                "16.43".to_string(),
                "16.44".to_string(),
                "16.45".to_string(),
                "16.46".to_string(),
                "16.47".to_string(),
                "16.48".to_string(),
                "16.49".to_string(),
                "16.50".to_string(),
                "16.51".to_string(),
                "16.52".to_string(),
                "16.53".to_string(),
                "16.54".to_string(),
                "16.55".to_string(),
                "16.56".to_string(),
                "16.57".to_string(),
                "16.58".to_string(),
            ],
        },
        Checks {
            name: "react@latest".to_string(),
            assertions: vec![
                "16.13.1".to_string(),
                "16.13".to_string(),
                "16.14".to_string(),
                "16.15".to_string(),
                "16.16".to_string(),
                "16.17".to_string(),
                "16.18".to_string(),
                "16.19".to_string(),
                "16.20".to_string(),
                "16.21".to_string(),
                "16.22".to_string(),
                "16.23".to_string(),
                "16.24".to_string(),
                "16.25".to_string(),
                "16.26".to_string(),
                "16.27".to_string(),
                "16.28".to_string(),
                "16.29".to_string(),
                "16.30".to_string(),
                "16.31".to_string(),
                "16.32".to_string(),
                "16.33".to_string(),
                "16.34".to_string(),
                "16.35".to_string(),
                "16.36".to_string(),
                "16.37".to_string(),
                "16.38".to_string(),
                "16.39".to_string(),
                "16.40".to_string(),
                "16.41".to_string(),
                "16.42".to_string(),
                "16.43".to_string(),
                "16.44".to_string(),
                "16.45".to_string(),
                "16.46".to_string(),
                "16.47".to_string(),
                "16.48".to_string(),
                "16.49".to_string(),
                "16.50".to_string(),
                "16.51".to_string(),
                "16.52".to_string(),
                "16.53".to_string(),
                "16.54".to_string(),
                "16.55".to_string(),
                "16.56".to_string(),
                "16.57".to_string(),
                "16.58".to_string(),
            ],
        },
        Checks {
            name: "react@>=10.0.0 <11.0.0".to_string(),
            assertions: vec![
                "10.0.0".to_string(),
                "10.0.1".to_string(),
                "10.0.2".to_string(),
                "10.0.3".to_string(),
                "10.0.4".to_string(),
                "10.0.5".to_string(),
                "10.0.6".to_string(),
                "10.0.7".to_string(),
                "10.0.8".to_string(),
                "10.0.9".to_string(),
                "10.0.10".to_string(),
                "10.0.11".to_string(),
                "10.0.12".to_string(),
                "10.0.13".to_string(),
                "10.0.14".to_string(),
                "10.0.15".to_string(),
                "10.0.16".to_string(),
                "10.0.17".to_string(),
                "10.0.18".to_string(),
                "10.0.19".to_string(),
                "10.0.20".to_string(),
                "10.0.21".to_string(),
                "10.0.22".to_string(),
                "10.0.23".to_string(),
                "10.0.24".to_string(),
                "10.0.25".to_string(),
                "10.0.26".to_string(),
                "10.0.27".to_string(),
                "10.0.28".to_string(),
                "10.0.29".to_string(),
                "10.0.30".to_string(),
                "10.0.31".to_string(),
                "10.0.32".to_string(),
                "10.0.33".to_string(),
                "10.0.34".to_string(),
                "10.0.35".to_string(),
                "10.0.36".to_string(),
                "10.0.37".to_string(),
                "10.0.38".to_string(),
                "10.0.39".to_string(),
                "10.0.40".to_string(),
                "10.0.41".to_string(),
                "10.0.42".to_string(),
                "10.0.43".to_string(),
                "10.0.44".to_string(),
            ],
        },
        Checks {
            name: "react@>=10.0.0,<11.0.0".to_string(),
            assertions: vec![
                "10.0.0".to_string(),
                "10.0.1".to_string(),
                "10.0.2".to_string(),
                "10.0.3".to_string(),
                "10.0.4".to_string(),
                "10.0.5".to_string(),
                "10.0.6".to_string(),
                "10.0.7".to_string(),
                "10.0.8".to_string(),
                "10.0.9".to_string(),
                "10.0.10".to_string(),
                "10.0.11".to_string(),
                "10.0.12".to_string(),
                "10.0.13".to_string(),
                "10.0.14".to_string(),
                "10.0.15".to_string(),
                "10.0.16".to_string(),
                "10.0.17".to_string(),
                "10.0.18".to_string(),
                "10.0.19".to_string(),
                "10.0.20".to_string(),
                "10.0.21".to_string(),
                "10.0.22".to_string(),
                "10.0.23".to_string(),
                "10.0.24".to_string(),
                "10.0.25".to_string(),
                "10.0.26".to_string(),
                "10.0.27".to_string(),
                "10.0.28".to_string(),
                "10.0.29".to_string(),
                "10.0.30".to_string(),
                "10.0.31".to_string(),
                "10.0.32".to_string(),
                "10.0.33".to_string(),
                "10.0.34".to_string(),
                "10.0.35".to_string(),
                "10.0.36".to_string(),
                "10.0.37".to_string(),
                "10.0.38".to_string(),
                "10.0.39".to_string(),
                "10.0.40".to_string(),
                "10.0.41".to_string(),
                "10.0.42".to_string(),
            ],
        },
        Checks {
            name: "react@>=10.0.0".to_string(),
            assertions: vec![
                "10.0.0".to_string(),
                "10.0.1".to_string(),
                "10.0.2".to_string(),
                "10.0.3".to_string(),
                "10.0.4".to_string(),
                "10.0.5".to_string(),
                "10.0.6".to_string(),
                "10.0.7".to_string(),
                "10.0.8".to_string(),
                "10.0.9".to_string(),
                "10.0.10".to_string(),
                "10.0.11".to_string(),
                "10.0.12".to_string(),
                "10.0.13".to_string(),
                "10.0.14".to_string(),
                "10.0.15".to_string(),
                "10.0.16".to_string(),
                "10.0.17".to_string(),
                "10.0.18".to_string(),
                "10.0.19".to_string(),
                "10.0.20".to_string(),
                "10.0.21".to_string(),
                "10.0.22".to_string(),
                "10.0.23".to_string(),
                "10.0.24".to_string(),
                "10.0.25".to_string(),
                "10.0.26".to_string(),
                "10.0.27".to_string(),
                "10.0.28".to_string(),
                "10.0.29".to_string(),
                "10.0.30".to_string(),
                "10.0.31".to_string(),
                "10.0.32".to_string(),
                "10.0.33".to_string(),
                "10.0.34".to_string(),
                "10.0.35".to_string(),
                "10.0.36".to_string(),
                "10.0.37".to_string(),
                "10.0.38".to_string(),
                "10.0.39".to_string(),
                "10.0.40".to_string(),
                "10.0.41".to_string(),
                "10.0.42".to_string(),
            ],
        },
        Checks {
            name: "react@<10.0.0".to_string(),
            assertions: vec![
                "9.0.0".to_string(),
                "9.0.1".to_string(),
                "9.0.2".to_string(),
                "9.0.3".to_string(),
                "9.0.4".to_string(),
                "9.0.5".to_string(),
                "9.0.6".to_string(),
                "9.0.7".to_string(),
                "9.0.8".to_string(),
                "9.0.9".to_string(),
                "9.0.10".to_string(),
                "9.0.11".to_string(),
                "9.0.12".to_string(),
                "9.0.13".to_string(),
                "9.0.14".to_string(),
                "9.0.15".to_string(),
                "9.0.16".to_string(),
                "9.0.17".to_string(),
                "9.0.18".to_string(),
                "9.0.19".to_string(),
                "9.0.20".to_string(),
                "9.0.21".to_string(),
                "9.0.22".to_string(),
                "9.0.23".to_string(),
                "9.0.24".to_string(),
                "9.0.25".to_string(),
                "9.0.26".to_string(),
                "9.0.27".to_string(),
                "9.0.28".to_string(),
                "9.0.29".to_string(),
                "9.0.30".to_string(),
                "9.0.31".to_string(),
                "9.0.32".to_string(),
                "9.0.33".to_string(),
                "9.0.34".to_string(),
                "9.0.35".to_string(),
                "9.0.36".to_string(),
                "9.0.37".to_string(),
                "9.0.38".to_string(),
                "9.0.39".to_string(),
                "9.0.40".to_string(),
                "9.0.41".to_string(),
                "9.0.42".to_string(),
            ],
        },
        Checks {
            name: "react@10.0.0||11".to_string(),
            assertions: vec![
                "10.0.0".to_string(),
                "10.0.1".to_string(),
                "10.0.2".to_string(),
                "10.0.3".to_string(),
                "10.0.4".to_string(),
                "10.0.5".to_string(),
                "10.0.6".to_string(),
                "10.0.7".to_string(),
                "10.0.8".to_string(),
                "10.0.9".to_string(),
                "10.0.10".to_string(),
                "10.0.11".to_string(),
                "10.0.12".to_string(),
                "10.0.13".to_string(),
                "10.0.14".to_string(),
                "10.0.15".to_string(),
                "10.0.16".to_string(),
                "10.0.17".to_string(),
                "10.0.18".to_string(),
                "10.0.19".to_string(),
                "10.0.20".to_string(),
                "10.0.21".to_string(),
                "10.0.22".to_string(),
                "10.0.23".to_string(),
                "10.0.24".to_string(),
                "10.0.25".to_string(),
                "10.0.26".to_string(),
                "10.0.27".to_string(),
                "10.0.28".to_string(),
                "10.0.29".to_string(),
                "10.0.30".to_string(),
                "10.0.31".to_string(),
                "10.0.32".to_string(),
                "10.0.33".to_string(),
                "10.0.34".to_string(),
                "10.0.35".to_string(),
                "10.0.36".to_string(),
                "10.0.37".to_string(),
                "10.0.38".to_string(),
                "10.0.39".to_string(),
                "10.0.40".to_string(),
                "10.0.41".to_string(),
                "10.0.42".to_string(),
            ],
        },
        Checks {
            name: "react@>=10.0.0||<=11".to_string(),
            assertions: vec![
                "10.0.0".to_string(),
                "10.0.1".to_string(),
                "10.0.2".to_string(),
                "10.0.3".to_string(),
                "10.0.4".to_string(),
                "10.0.5".to_string(),
                "10.0.6".to_string(),
                "10.0.7".to_string(),
                "10.0.8".to_string(),
                "10.0.9".to_string(),
                "10.0.10".to_string(),
                "10.0.11".to_string(),
                "10.0.12".to_string(),
                "10.0.13".to_string(),
                "10.0.14".to_string(),
                "10.0.15".to_string(),
                "10.0.16".to_string(),
                "10.0.17".to_string(),
                "10.0.18".to_string(),
                "10.0.19".to_string(),
                "10.0.20".to_string(),
                "10.0.21".to_string(),
                "10.0.22".to_string(),
                "10.0.23".to_string(),
                "10.0.24".to_string(),
                "10.0.25".to_string(),
                "10.0.26".to_string(),
                "10.0.27".to_string(),
                "10.0.28".to_string(),
                "10.0.29".to_string(),
                "10.0.30".to_string(),
                "10.0.31".to_string(),
                "10.0.32".to_string(),
                "10.0.33".to_string(),
                "10.0.34".to_string(),
                "10.0.35".to_string(),
                "10.0.36".to_string(),
                "10.0.37".to_string(),
                "10.0.38".to_string(),
                "10.0.39".to_string(),
                "10.0.40".to_string(),
                "10.0.41".to_string(),
                "10.0.42".to_string(),
            ],
        },
        Checks {
          name: "react@>=10.0.0<13||>=11<15".to_string(),
          assertions: vec![
              "10.0.0".to_string(),
              "10.0.1".to_string(),
              "10.0.2".to_string(),
              "10.0.3".to_string(),
              "10.0.4".to_string(),
              "10.0.5".to_string(),
              "10.0.6".to_string(),
              "10.0.7".to_string(),
              "10.0.8".to_string(),
              "10.0.9".to_string(),
              "10.0.10".to_string(),
              "10.0.11".to_string(),
              "10.0.12".to_string(),
              "10.0.13".to_string(),
              "10.0.14".to_string(),
              "10.0.15".to_string(),
              "10.0.16".to_string(),
              "10.0.17".to_string(),
              "10.0.18".to_string(),
              "10.0.19".to_string(),
              "10.0.20".to_string(),
              "10.0.21".to_string(),
              "10.0.22".to_string(),
              "10.0.23".to_string(),
              "10.0.24".to_string(),
              "10.0.25".to_string(),
              "10.0.26".to_string(),
              "10.0.27".to_string(),
              "10.0.28".to_string(),
              "10.0.29".to_string(),
              "10.0.30".to_string(),
              "10.0.31".to_string(),
              "10.0.32".to_string(),
              "10.0.33".to_string(),
              "10.0.34".to_string(),
              "10.0.35".to_string(),
              "10.0.36".to_string(),
              "10.0.37".to_string(),
              "10.0.38".to_string(),
              "10.0.39".to_string(),
              "10.0.40".to_string(),
              "10.0.41".to_string(),
              "10.0.42".to_string(),
          ],
      },
    ];

    for check in checks {
        let package = Package::new(&check.name);

        println!("Package version: {}", package.raw_version);
        println!("Parsed groups count: {}", package.version.inner.len());
        println!(
          "Parsed constraints count: {}",
          package.version.inner.iter().fold(0, |acc, group| acc + group.constraints.len())
        );
        for assertion in check.assertions {
            println!("{}: {}", assertion, package.version.satisfies(&assertion));
        }
        println!("--------------------------------------------\n\n\n");
    }
}
