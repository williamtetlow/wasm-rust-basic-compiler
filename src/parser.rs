use pest::{self, Parser};

use crate::ast::{Node, Operator};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct CalcParser;

pub fn parse(source: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let pairs = CalcParser::parse(Rule::Program, source)?;

    for pair in pairs {
        if let Rule::Expr = pair.as_rule() {
            ast.push(build_ast_from_expr(pair));
        }
    }

    Ok(ast)
}

fn build_ast_from_term(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::Int => {
            let istr = pair.as_str();
            let (sign, istr) = match &istr[..1] {
                "-" => (-1, &istr[1..]),
                _ => (1, istr),
            };
            let int: i32 = istr.parse().unwrap();

            Node::Int(sign * int)
        }
        Rule::Expr => build_ast_from_expr(pair),
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::Expr => {
            let mut pair = pair.into_inner();
            let op = pair.next().unwrap();
            let lhspair = pair.next().unwrap();
            let lhs = build_ast_from_term(lhspair);
            let rhspair = pair.next().unwrap();
            let rhs = build_ast_from_term(rhspair);

            parse_expr(op, lhs, rhs)
        }
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

fn parse_expr(pair: pest::iterators::Pair<Rule>, lhs: Node, rhs: Node) -> Node {
    Node::Expression {
        op: match pair.as_str() {
            "add" => Operator::Plus,
            "subtract" => Operator::Minus,
            "divide" => Operator::Divide,
            "multiply" => Operator::Multiply,
            _ => unreachable!(),
        },
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}
