use colored::Colorize;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
  Number,
  Text,
  Char,
  Indent,
  Bool,
  Identifier,
  Symbol,
  Keyword,
  Operator,
  Whitespace,
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
      Indent     => write!(f, "Indent"),
      Bool       => write!(f, "Bool"),
      Identifier => write!(f, "Identifier"),
      Symbol     => write!(f, "Symbol"),
      Keyword    => write!(f, "Keyword"),
      Operator   => write!(f, "Operator"),
      Whitespace => write!(f, "Whitespace"),
      EOL        => write!(f, "EOL"),
      EOF        => write!(f, "EOF"),
    }
  }
}



#[derive(Debug, Clone, PartialEq)]
pub struct Pos(pub (usize, String), pub (usize, usize));

impl fmt::Display for Pos {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let linepad = format!("{:5} │", " ").dimmed().bold();
    let lineno = format!("{:5} │ ", (self.0).0).dimmed().bold();
    let mut mark = (self.0).1[(self.1).0.saturating_sub(1) .. (self.1).1].to_string();

    if mark.split_whitespace().count() == 0 {
      mark = format!("{:─>count$}", ">".bold(), count=mark.len());
    } else {
      mark = format!("{}", mark.bold());
    }

    let mut arrows = format!("{: <count$}", " ", count=(self.1).0);

    for _ in 0 .. (self.1).1 - (self.1).0 + 1 {
      arrows.push('^')
    }

    write!(f, "\n{}\n{}{}{}{}\n{}{}",
      linepad,
      lineno, &(self.0).1[..(self.1).0.saturating_sub(1)], mark, &(self.0).1[(self.1).1..],
      linepad,
      arrows.bold()
    )
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
    write!(
      f,
      "{}",
      Pos(
        (self.line.0,  self.line.1.clone()),
        (self.slice.0, self.slice.1)
      )
    )
  }
}