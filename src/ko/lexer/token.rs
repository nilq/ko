use colored::Colorize;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
  Number,
  Char,
  Bool,
  Text,
  Identifier,
  Symbol,
  Keyword,
  Operator,
  EOL,
  EOF,
}

impl fmt::Display for TokenType {
  fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::TokenType::*;

    match *self {
      Number     => write!(f, "Number"),
      Text       => write!(f, "Text"),
      Char       => write!(f, "Char"),
      Bool       => write!(f, "Bool"),
      Identifier => write!(f, "Identifier"),
      Symbol     => write!(f, "Symbol"),
      Keyword    => write!(f, "Keyword"),
      Operator   => write!(f, "Operator"),
      EOL        => write!(f, "EOL"),
      EOF        => write!(f, "EOF"),
    }
  }
}