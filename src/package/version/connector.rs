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

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connector_from_str() {
        assert_eq!(",".parse::<Connector>().unwrap(), Connector::And);
        assert_eq!("|".parse::<Connector>().unwrap(), Connector::Or);
        assert_eq!(" ".parse::<Connector>().unwrap(), Connector::And);
        assert_eq!("||".parse::<Connector>().unwrap(), Connector::Or);
        assert_eq!("".parse::<Connector>().unwrap(), Connector::And);
    }
}
