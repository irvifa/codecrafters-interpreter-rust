// ast.rs
use crate::tokenizer::TokenType;
use std::fmt;

pub enum Expr {
    Literal(LiteralValue),
    // We'll add more expression types later
    Grouping(Box<Expr>),
    Unary(TokenType, Box<Expr>),
    Binary(Box<Expr>, TokenType, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Literal(value) => write!(f, "{}", value),
            Expr::Grouping(expr) => write!(f, "(group {})", expr),
            Expr::Unary(operator, expr) => {
                write!(f, "({} {})", operator.to_string_for_parse(), expr)
            }
            Expr::Binary(left, operator, right) => {
                write!(f, "({} {} {})", operator.to_string_for_parse(), left, right)
            }
        }
    }
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralValue::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{:.1}", n)
                } else {
                    write!(f, "{}", n)
                }
            }
            LiteralValue::String(s) => write!(f, "{}", s), // Remove quotes for output
            LiteralValue::Bool(b) => write!(f, "{}", b),
            LiteralValue::Nil => write!(f, "nil"),
        }
    }
}
