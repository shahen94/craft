use super::{connector::Connector, constraint::VersionConstraint, contracts::Satisfies};

// ─── VersionGroup ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct VersionGroup {
    pub constraints: Vec<VersionConstraint>,
    pub connector: Connector,
}

// ───────────────────────────────────────────────────────────────────────────────

impl VersionGroup {
    pub fn new(constraints: Vec<VersionConstraint>, connector: Connector) -> Self {
        Self {
            constraints,
            connector,
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────

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

impl Satisfies for VersionGroup {
    fn satisfies(&self, version: &str) -> bool {
        match self.connector {
            Connector::And => self
                .constraints
                .iter()
                .all(|constraint| constraint.satisfies(version)),
            Connector::Or => self
                .constraints
                .iter()
                .any(|constraint| constraint.satisfies(version)),
        }
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_group_to_string() {
        let group = VersionGroup::new(
            vec![
                VersionConstraint::parse("1.0.0"),
                VersionConstraint::parse("2.0.0"),
            ],
            Connector::And,
        );

        assert_eq!(group.to_string(), "1.0.0 2.0.0");

        let group = VersionGroup::new(
            vec![
                VersionConstraint::parse("1.0.0"),
                VersionConstraint::parse("2.0.0"),
            ],
            Connector::Or,
        );

        assert_eq!(group.to_string(), "(1.0.0 2.0.0)");
    }

    #[test]
    fn test_version_group_satisfies() {
        let group = VersionGroup::new(
            vec![
                VersionConstraint::parse("1.0.0"),
                VersionConstraint::parse("2.0.0"),
            ],
            Connector::And,
        );

        assert!(!group.satisfies("1.0.0"));
        assert!(!group.satisfies("2.0.0"));
        assert!(!group.satisfies("3.0.0"));

        let group = VersionGroup::new(
            vec![
                VersionConstraint::parse("1.0.0"),
                VersionConstraint::parse("2.0.0"),
            ],
            Connector::Or,
        );

        assert!(group.satisfies("1.0.0"));
        assert!(group.satisfies("2.0.0"));
        assert!(!group.satisfies("3.0.0"));
    }
}
