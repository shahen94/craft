use regex::Regex;

use crate::package::version::{connector::Connector, constants::LINEAR_RANGE_REGEX};

use super::{
    constants::RANGE_REGEX, constraint::VersionConstraint, contracts::{Satisfies, Version}, field::VersionField, group::VersionGroup, operator::Operator
};

// ─── VersionImpl ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct VersionImpl {
    inner: Vec<VersionGroup>,
}

// ─── ToString ────────────────────────────────────────────────────────────────

impl ToString for VersionImpl {
    fn to_string(&self) -> String {
        self.inner
            .iter()
            .map(|constraint| constraint.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

// ─── Impl ────────────────────────────────────────────────────────────────────

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

        let captures = match Regex::new(LINEAR_RANGE_REGEX).unwrap().captures(version) {
            Some(captures) => captures,
            None => {
                let constraint = VersionConstraint::parse(version);

                return vec![VersionGroup::new(vec![constraint], Connector::And)]
            }
        };

        let mut constraints = vec![];

        let mut start_major = VersionField::Wildcard;
        let mut start_minor = VersionField::Wildcard;
        let mut start_patch = VersionField::Wildcard;

        let mut end_major = VersionField::Wildcard;
        let mut end_minor = VersionField::Wildcard;
        let mut end_patch = VersionField::Wildcard;

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

        constraints.push(VersionConstraint {
            operator: Operator::GreaterThanOrEqual,
            major: start_major,
            minor: start_minor,
            patch: start_patch,
            pre_release: None,
            build: None,
        });

        constraints.push(VersionConstraint {
            operator: Operator::LessThanOrEqual,
            major: end_major,
            minor: end_minor,
            patch: end_patch,
            pre_release: None,
            build: None,
        });

        return vec![VersionGroup::new(constraints, Connector::And)]

    }

    fn parse_range(version: &str) -> Vec<VersionConstraint> {
        if version.contains("-") {
            let parts = version.split("-").collect::<Vec<_>>();
            let mut constraints = vec![];

            for part in parts {
                constraints.push(VersionConstraint::parse(part));
            }

            return constraints;
        }

        let mut start_operator = Operator::Equal;
        let mut start_major = VersionField::Wildcard;
        let mut start_minor = VersionField::Wildcard;
        let mut start_patch = VersionField::Wildcard;

        let mut end_operator = Operator::Equal;
        let mut end_major = VersionField::Wildcard;
        let mut end_minor = VersionField::Wildcard;
        let mut end_patch = VersionField::Wildcard;

        let regex = Regex::new(RANGE_REGEX).unwrap();

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

// ─── Version ───────────────────────────────────────────────────────────────

impl Version for VersionImpl {
    fn new(version: &str) -> Self {
        let version = match version.trim() {
            "" => "*",
            version => version,
        };
        let inner = Self::parse_constraints(&version);

        Self { inner }
    }

    fn is_exact(&self) -> bool {
        if self.inner.len() != 1 {
            return false;
        }

        let group = &self.inner[0];

        if group.constraints.len() != 1 {
            return false;
        }

        let constraint = &group.constraints[0];

        if constraint.operator != Operator::Equal {
            return false;
        }

        match (&constraint.major, &constraint.minor, &constraint.patch) {
            (VersionField::Exact(_), VersionField::Exact(_), VersionField::Exact(_)) => true,
            _ => false,
        }
    }

    fn satisfies(&self, version: &str) -> bool {
        true
    }
}

// ─── Satisfies ───────────────────────────────────────────────────────────────

impl Satisfies for VersionImpl {
    fn satisfies(&self, version: &str) -> bool {
        true
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_new() {
        let version = VersionImpl::new("1.0.0");
        assert_eq!(version.inner.len(), 1);
        assert_eq!(version.inner[0].constraints.len(), 1);
        assert_eq!(version.inner[0].constraints[0].operator, Operator::Equal);
        assert_eq!(version.inner[0].constraints[0].major, VersionField::Exact(1));
        assert_eq!(version.inner[0].constraints[0].minor, VersionField::Exact(0));
        assert_eq!(version.inner[0].constraints[0].patch, VersionField::Exact(0));

        let version = VersionImpl::new("1.0.0 || 2.0.0");
        assert_eq!(version.inner.len(), 2);
        assert_eq!(version.inner[0].constraints.len(), 1);
        assert_eq!(version.inner[0].constraints[0].operator, Operator::Equal);
        assert_eq!(version.inner[0].constraints[0].major, VersionField::Exact(1));
        assert_eq!(version.inner[0].constraints[0].minor, VersionField::Exact(0));
        assert_eq!(version.inner[0].constraints[0].patch, VersionField::Exact(0));
        assert_eq!(version.inner[1].constraints.len(), 1);
        assert_eq!(version.inner[1].constraints[0].operator, Operator::Equal);
        assert_eq!(version.inner[1].constraints[0].major, VersionField::Exact(2));
        assert_eq!(version.inner[1].constraints[0].minor, VersionField::Exact(0));
        assert_eq!(version.inner[1].constraints[0].patch, VersionField::Exact(0));

        let version = VersionImpl::new(">=1.0.0 <2.0.0");
        assert_eq!(version.inner.len(), 1);
        assert_eq!(version.inner[0].constraints.len(), 2);
        assert_eq!(version.inner[0].constraints[0].operator, Operator::GreaterThanOrEqual);
        assert_eq!(version.inner[0].constraints[0].major, VersionField::Exact(1));
        assert_eq!(version.inner[0].constraints[0].minor, VersionField::Exact(0));
        assert_eq!(version.inner[0].constraints[0].patch, VersionField::Exact(0));
        assert_eq!(version.inner[0].constraints[1].operator, Operator::LessThan);
        assert_eq!(version.inner[0].constraints[1].major, VersionField::Exact(2));
        assert_eq!(version.inner[0].constraints[1].minor, VersionField::Exact(0));
        assert_eq!(version.inner[0].constraints[1].patch, VersionField::Exact(0));

        let version = VersionImpl::new("1.0.0 - 2.0.0");
        assert_eq!(version.inner.len(), 1);
        assert_eq!(version.inner[0].constraints.len(), 2);
        assert_eq!(version.inner[0].constraints[0].operator, Operator::GreaterThanOrEqual);
        assert_eq!(version.inner[0].constraints[0].major, VersionField::Exact(1));
        assert_eq!(version.inner[0].constraints[0].minor, VersionField::Exact(0));
        assert_eq!(version.inner[0].constraints[0].patch, VersionField::Exact(0));
        assert_eq!(version.inner[0].constraints[1].operator, Operator::LessThanOrEqual);
        assert_eq!(version.inner[0].constraints[1].major, VersionField::Exact(2));
        assert_eq!(version.inner[0].constraints[1].minor, VersionField::Exact(0));
        assert_eq!(version.inner[0].constraints[1].patch, VersionField::Exact(0));
    }

    #[test]
    fn test_version_is_exact() {
        let version = VersionImpl::new("1.0.0");
        assert_eq!(version.is_exact(), true);

        let version = VersionImpl::new("1.0.0 || 2.0.0");
        assert_eq!(version.is_exact(), false);

        let version = VersionImpl::new(">=1.0.0 <2.0.0");
        assert_eq!(version.is_exact(), false);

        let version = VersionImpl::new("1.0.0 - 2.0.0");
        assert_eq!(version.is_exact(), false);

        let version = VersionImpl::new("1.*.*");
        assert_eq!(version.is_exact(), false);
    }

    #[test]
    fn test_version_satisfies() {
        
    }

    #[test]
    fn test_version_satisfies_range() {
        
    }
}