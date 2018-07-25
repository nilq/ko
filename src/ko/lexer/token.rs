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

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  pub token_type: TokenType,
  pub line:       (usize, String),
  pub slice:      (usize, usize),
  pub lexeme:     String,
}


impl Token {
  pub fn new(token_type: TokenType, line: (usize, String), slice: (usize, usize), lexeme: &str) -> Self {
    Token {
      token_type,
      line,
      slice,
      lexeme: lexeme.to_string()
    }
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let linepad = format!("{:5} │", " ").blue().bold();
    let lineno = format!("{:5} │ ", self.line.0).blue().bold();
    let mut mark = self.line.1[self.slice.0.saturating_sub(1) .. self.slice.1].to_string();

    if mark.split_whitespace().count() == 0 {
      mark = format!("{:─>count$}", ">".bold().magenta(), count=mark.len());
    } else {
      mark = format!("{}", mark.bold().magenta());
    }

    let mut arrows = format!("{: <count$}", " ", count=self.slice.0);

    for _ in 0 .. self.slice.1 - self.slice.0 + 1 {
      arrows.push('^')
    }

    write!(f, "\n{}\n{}{}{}{}\n{}{}",
      linepad,
      lineno, &self.line.1[..self.slice.0.saturating_sub(1)], mark, &self.line.1[self.slice.1..],
      linepad,
      arrows.bold().magenta()
    )
  }
}