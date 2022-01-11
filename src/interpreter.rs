use crate::{
    ast::{Node, Operator},
    parser,
};

pub type Result<T> = anyhow::Result<T>;

pub trait Compile {
    type Output;

    fn from_ast(ast: Vec<Node>) -> Self::Output;

    fn from_source(source: &str) -> Self::Output {
        let ast: Vec<Node> = parser::parse(source).unwrap();
        Self::from_ast(ast)
    }
}

pub struct Interpreter;

impl Compile for Interpreter {
    type Output = Result<i32>;

    fn from_ast(ast: Vec<Node>) -> Self::Output {
        let mut ret = 0i32;
        let evaluator = Eval::new();
        for node in ast {
            ret += evaluator.eval(&node);
        }
        Ok(ret)
    }
}

struct Eval;

impl Eval {
    pub fn new() -> Self {
        Self
    }

    pub fn eval(&self, node: &Node) -> i32 {
        match node {
            Node::Int(n) => *n,
            Node::Expression { op, lhs, rhs } => {
                let lhs_ret = self.eval(lhs);
                let rhs_ret = self.eval(rhs);

                match op {
                    Operator::Plus => lhs_ret + rhs_ret,
                    Operator::Minus => lhs_ret - rhs_ret,
                    Operator::Multiply => lhs_ret * rhs_ret,
                    Operator::Divide => lhs_ret / rhs_ret,
                }
            }
        }
    }
}
