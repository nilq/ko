use std::rc::Rc;
use std::fmt;
use std::collections::HashMap;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum StatementNode {
  Expression(Expression),
  Table(Expression, HashMap<String, Expression>)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
  pub node: StatementNode,
  pub pos:  Pos,
}

impl Statement {
  pub fn new(node: StatementNode, pos: Pos) -> Self {
    Statement {
      node,
      pos,
    }
  }
}



#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode {
  Number(f64),
  Text(String),
  Char(char),
  Bool(bool),
  Identifier(String),
  Binary(Rc<Expression>, Operator, Rc<Expression>),
  EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
  pub node: ExpressionNode,
  pub pos:  Pos,
}

impl Expression {
  pub fn new(node: ExpressionNode, pos: Pos) -> Self {
    Expression {
      node,
      pos,
    }
  }
}



#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Pow,
  Eq,
  Lt,
  Gt,
  NEq,
  LtEq,
  GtEq,
}

impl Operator {
  pub fn from_str(operator: &str) -> Option<(Operator, u8)> {
    use self::Operator::*;

    let op_prec = match operator {
      "==" => (Eq,     1),
      "<"  => (Lt,     1),
      ">"  => (Gt,     1),
      "!=" => (NEq,    1),
      "<=" => (LtEq,   1),
      ">=" => (GtEq,   1),
      "+"  => (Add,    2),
      "-"  => (Sub,    2),
      "*"  => (Mul,    3),
      "/"  => (Div,    3),
      "%"  => (Mod,    3),
      "^"  => (Pow,    4),
      _    => return None,
    };

    Some(op_prec)
  }
}

impl fmt::Display for Operator {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::Operator::*;

    let t = match *self {
      Add    => "+",
      Sub    => "-",
      Pow    => "^",
      Mul    => "*",
      Div    => "/",
      Mod    => "%",
      Eq     => "==",
      Lt     => "<",
      Gt     => ">",
      NEq    => "!=",
      LtEq   => "<=",
      GtEq   => ">=",
    };

    write!(f, "{}", t)
  }
}