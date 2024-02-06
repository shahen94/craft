use std::str::FromStr;

// ─── Operator ──────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Tilde,
    Caret,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
}

// ─────────────────────────────────────────────────────────────────────────────

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "~" => Ok(Operator::Tilde),
            "^" => Ok(Operator::Caret),
            "~>" => Ok(Operator::Tilde),
            ">" => Ok(Operator::GreaterThan),
            ">=" => Ok(Operator::GreaterThanOrEqual),
            "<" => Ok(Operator::LessThan),
            "<=" => Ok(Operator::LessThanOrEqual),
            "=" => Ok(Operator::Equal),
            _ => Err(()),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────

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

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_from_str() {
        assert_eq!("~".parse::<Operator>().unwrap(), Operator::Tilde);
        assert_eq!("^".parse::<Operator>().unwrap(), Operator::Caret);
        assert_eq!(">".parse::<Operator>().unwrap(), Operator::GreaterThan);
        assert_eq!(
            ">=".parse::<Operator>().unwrap(),
            Operator::GreaterThanOrEqual
        );
        assert_eq!("<".parse::<Operator>().unwrap(), Operator::LessThan);
        assert_eq!("<=".parse::<Operator>().unwrap(), Operator::LessThanOrEqual);
        assert_eq!("=".parse::<Operator>().unwrap(), Operator::Equal);
        assert!("".parse::<Operator>().is_err());
    }

    #[test]
    fn test_operator_to_string() {
        assert_eq!(Operator::Tilde.to_string(), "~");
        assert_eq!(Operator::Caret.to_string(), "^");
        assert_eq!(Operator::GreaterThan.to_string(), ">");
        assert_eq!(Operator::GreaterThanOrEqual.to_string(), ">=");
        assert_eq!(Operator::LessThan.to_string(), "<");
        assert_eq!(Operator::LessThanOrEqual.to_string(), "<=");
        assert_eq!(Operator::Equal.to_string(), "=");
    }
}
