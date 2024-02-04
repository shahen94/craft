use std::str::FromStr;

// ─── Connector ───────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone)]
pub enum Connector {
    And,
    Or,
}

// ─────────────────────────────────────────────────────────────────────────────

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