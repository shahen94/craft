use super::{connector::Connector, constraint::VersionConstraint};

#[derive(Debug, Clone)]
pub struct VersionGroup {
    pub constraints: Vec<VersionConstraint>,
    pub connector: Connector,
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