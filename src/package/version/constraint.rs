use super::{constants::SEMVER_REGEX, contracts::Satisfies, field::VersionField, operator::Operator};

#[derive(Debug, Clone)]
pub struct VersionConstraint {
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
