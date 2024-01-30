use std::str::FromStr;

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
