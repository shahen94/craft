use super::{connector::Connector, constraint::VersionConstraint};

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
}
