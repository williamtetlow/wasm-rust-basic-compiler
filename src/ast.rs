use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operator {
    Plus,
    Minus,
    Divide,
    Multiply,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Divide => write!(f, "/"),
            Operator::Multiply => write!(f, "*"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Node {
    Int(i32),
    Expression {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Node::Int(n) => write!(f, "{}", n),
            Node::Expression { op, lhs, rhs } => write!(f, "{} {} {}", lhs, op, rhs),
        }
    }
}
