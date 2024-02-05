use super::{
    constants::SEMVER_REGEX, contracts::Satisfies, field::VersionField, operator::Operator,
};

// ─── VersionConstraint ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct VersionConstraint {
    pub operator: Operator,
    pub major: VersionField,
    pub minor: VersionField,
    pub patch: VersionField,
    pub pre_release: Option<String>,
    pub build: Option<String>,
}

// ─────────────────────────────────────────────────────────────────────────────

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
            .captures(&version.trim())
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

        match operator {
            Operator::Caret => {
                minor = VersionField::Wildcard;
                patch = VersionField::Wildcard;
            }
            Operator::Tilde => {
                patch = VersionField::Wildcard;
            }
            _ => {}
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

    fn satisfies_equal(&self, version: &str) -> bool {
        let version = VersionConstraint::parse(version);

        self.major.satisfies(&version.major.to_string())
            && self.minor.satisfies(&version.minor.to_string())
            && self.patch.satisfies(&version.patch.to_string())
            && self.pre_release == version.pre_release
            && self.build == version.build
    }

    fn satisfies_gte(&self, version: &str) -> bool {
        // Is Equal
        if self.satisfies_equal(&version) {
            return true;
        }

        let version = VersionConstraint::parse(version);

        let is_major_gt = self.major.is_gt(&version.major.to_string());
        let is_minor_gt = self.minor.is_gte(&version.minor.to_string());
        let is_patch_gt = self.patch.is_gte(&version.patch.to_string());

        if is_major_gt {
            return true;
        }

        // Major is not gt, then it should be Equal

        let is_major_satisfies = self.major.satisfies(&version.major.to_string());

        // If major is satisfies, then minor should be gt
        if is_major_satisfies && is_minor_gt {
            return true;
        }

        // Then at least patch should be gte

        let is_minor_satisfies = self.minor.satisfies(&version.minor.to_string());

        if is_major_satisfies && is_minor_satisfies && is_patch_gt {
            return true;
        }

        false
    }

    fn satisfies_gt(&self, version: &str) -> bool {
        if self.satisfies_equal(&version) {
            return false;
        }

        let version = VersionConstraint::parse(version);

        let is_major_gt = self.major.is_gt(&version.major.to_string());
        let is_minor_gt = self.minor.is_gte(&version.minor.to_string());
        let is_patch_gt = self.patch.is_gte(&version.patch.to_string());

        if is_major_gt {
            return true;
        }

        // Major is not gt, then it should be Equal
        let is_major_satisfies = self.major.satisfies(&version.major.to_string());

        // If major is satisfies, then minor should be gt
        if is_major_satisfies && is_minor_gt {
            return true;
        }

        let is_minor_satisfies = self.minor.satisfies(&version.minor.to_string());

        if is_major_satisfies && is_minor_satisfies && is_patch_gt {
            return true;
        }

        false
    }

    fn satisfies_lte(&self, version: &str) -> bool {
        if self.satisfies_equal(&version) {
            return true;
        }

        let version = VersionConstraint::parse(version);

        let is_major_lt = self.major.is_lt(&version.major.to_string());
        let is_minor_lt = self.minor.is_lte(&version.minor.to_string());
        let is_patch_lt = self.patch.is_lte(&version.patch.to_string());

        if is_major_lt {
            return true;
        }

        // Major is not lt, then it should be Equal
        let is_major_satisfies = self.major.satisfies(&version.major.to_string());

        // If major is satisfies, then minor should be lt
        if is_major_satisfies && is_minor_lt {
            return true;
        }

        let is_minor_satisfies = self.minor.satisfies(&version.minor.to_string());

        if is_major_satisfies && is_minor_satisfies && is_patch_lt {
            return true;
        }

        false
    }

    fn satisfies_lt(&self, version: &str) -> bool {
        if self.satisfies_equal(&version) {
            return false;
        }
        let version = VersionConstraint::parse(version);

        let is_major_lt = self.major.is_lt(&version.major.to_string());
        let is_minor_lt = self.minor.is_lte(&version.minor.to_string());
        let is_patch_lt = self.patch.is_lte(&version.patch.to_string());

        if is_major_lt {
            return true;
        }

        // Major is not lt, then it should be Equal
        let is_major_satisfies = self.major.satisfies(&version.major.to_string());

        // If major is satisfies, then minor should be lt
        if is_major_satisfies && is_minor_lt {
            return true;
        }

        let is_minor_satisfies = self.minor.satisfies(&version.minor.to_string());

        if is_major_satisfies && is_minor_satisfies && is_patch_lt {
            return true;
        }

        false
    }
}

// ─────────────────────────────────────────────────────────────────────────────

impl Satisfies for VersionConstraint {
    fn satisfies(&self, version: &str) -> bool {
        match self.operator {
            Operator::Caret | Operator::Equal | Operator::Tilde => self.satisfies_equal(version),
            Operator::GreaterThan => self.satisfies_gt(version),
            Operator::GreaterThanOrEqual => self.satisfies_gte(version),
            Operator::LessThan => self.satisfies_lt(version),
            Operator::LessThanOrEqual => self.satisfies_lte(version),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────

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

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_constraint_parse() {
        let version = VersionConstraint::parse("1.0.0");
        assert_eq!(version.major, VersionField::Exact(1));
        assert_eq!(version.minor, VersionField::Exact(0));
        assert_eq!(version.patch, VersionField::Exact(0));
        assert_eq!(version.operator, Operator::Equal);
        assert_eq!(version.pre_release, None);
        assert_eq!(version.build, None);

        let version = VersionConstraint::parse("=1.0.0");
        assert_eq!(version.major, VersionField::Exact(1));
        assert_eq!(version.minor, VersionField::Exact(0));
        assert_eq!(version.patch, VersionField::Exact(0));
        assert_eq!(version.operator, Operator::Equal);
        assert_eq!(version.pre_release, None);
        assert_eq!(version.build, None);

        let version = VersionConstraint::parse("^1.0.0-alpha");
        assert_eq!(version.major, VersionField::Exact(1));
        assert_eq!(version.minor, VersionField::Wildcard);
        assert_eq!(version.patch, VersionField::Wildcard);
        assert_eq!(version.operator, Operator::Caret);
        assert_eq!(version.pre_release, Some("alpha".to_string()));
        assert_eq!(version.build, None);

        let version = VersionConstraint::parse("~1.0.0+build");
        assert_eq!(version.major, VersionField::Exact(1));
        assert_eq!(version.minor, VersionField::Exact(0));
        assert_eq!(version.patch, VersionField::Wildcard);
        assert_eq!(version.operator, Operator::Tilde);
        assert_eq!(version.pre_release, None);
        assert_eq!(version.build, Some("build".to_string()));

        let version = VersionConstraint::parse("1.0.0-alpha+build");
        assert_eq!(version.major, VersionField::Exact(1));
        assert_eq!(version.minor, VersionField::Exact(0));
        assert_eq!(version.patch, VersionField::Exact(0));
        assert_eq!(version.operator, Operator::Equal);
        assert_eq!(version.pre_release, Some("alpha".to_string()));
        assert_eq!(version.build, Some("build".to_string()));

        let version = VersionConstraint::parse("=1.0.0");
        assert_eq!(version.major, VersionField::Exact(1));
        assert_eq!(version.minor, VersionField::Exact(0));
        assert_eq!(version.patch, VersionField::Exact(0));
        assert_eq!(version.operator, Operator::Equal);
        assert_eq!(version.pre_release, None);
        assert_eq!(version.build, None);
    }

    #[test]
    fn test_version_constraint_to_string() {
        let version = VersionConstraint {
            major: VersionField::Exact(1),
            minor: VersionField::Exact(0),
            patch: VersionField::Exact(0),
            operator: Operator::Equal,
            pre_release: None,
            build: None,
        };
        assert_eq!(version.to_string(), "1.0.0");

        let version = VersionConstraint {
            major: VersionField::Exact(1),
            minor: VersionField::Exact(0),
            patch: VersionField::Exact(0),
            operator: Operator::GreaterThan,
            pre_release: None,
            build: None,
        };
        assert_eq!(version.to_string(), ">1.0.0");

        let version = VersionConstraint {
            major: VersionField::Exact(1),
            minor: VersionField::Exact(0),
            patch: VersionField::Exact(0),
            operator: Operator::GreaterThan,
            pre_release: Some("alpha".to_string()),
            build: None,
        };
        assert_eq!(version.to_string(), ">1.0.0-alpha");

        let version = VersionConstraint {
            major: VersionField::Exact(1),
            minor: VersionField::Exact(0),
            patch: VersionField::Exact(0),
            operator: Operator::GreaterThan,
            pre_release: Some("alpha".to_string()),
            build: Some("build".to_string()),
        };
        assert_eq!(version.to_string(), ">1.0.0-alpha+build");
    }

    #[test]
    fn test_satisfies() {
        let version = VersionConstraint::parse("1.0.0");
        assert_eq!(version.satisfies("1.0.0"), true);
        assert_eq!(version.satisfies("1.0.1"), false);
        assert_eq!(version.satisfies("1.1.0"), false);
        assert_eq!(version.satisfies("2.0.0"), false);

        let version = VersionConstraint::parse("=1.0.0");
        assert_eq!(version.satisfies("1.0.0"), true);
        assert_eq!(version.satisfies("1.0.1"), false);
        assert_eq!(version.satisfies("1.1.0"), false);
        assert_eq!(version.satisfies("2.0.0"), false);

        let version = VersionConstraint::parse("^1.0.0");
        assert_eq!(version.satisfies("1.0.0"), true);
        assert_eq!(version.satisfies("1.0.1"), true);
        assert_eq!(version.satisfies("1.1.0"), true);
        assert_eq!(version.satisfies("2.0.0"), false);

        let version = VersionConstraint::parse("~1.0.0");
        assert_eq!(version.satisfies("1.0.0"), true);
        assert_eq!(version.satisfies("1.0.1"), true);
        assert_eq!(version.satisfies("1.1.0"), false);
        assert_eq!(version.satisfies("2.0.0"), false);

        let version = VersionConstraint::parse("1.0.0-alpha");
        assert_eq!(version.satisfies("1.0.0-alpha"), true);
        assert_eq!(version.satisfies("1.0.0-beta"), false);
        assert_eq!(version.satisfies("1.0.0"), false);

        let version = VersionConstraint::parse("1.0.0+build");
        assert_eq!(version.satisfies("1.0.0+build"), true);
        assert_eq!(version.satisfies("1.0.0"), false);
    }
}
