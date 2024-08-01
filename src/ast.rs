// ast.rs
use std::fmt;

pub enum Expr {
    Literal(LiteralValue),
    // We'll add more expression types later
}

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
        }
    }
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralValue::Number(n) => {
                // Check if the number is an integer
                if n.fract() == 0.0 {
                    write!(f, "{:.1}", n) // Print with one decimal place
                } else {
                    write!(f, "{}", n) // Print as is for floats
                }
            },
            LiteralValue::String(s) => write!(f, "\"{}\"", s),
            LiteralValue::Bool(b) => write!(f, "{}", b),
            LiteralValue::Nil => write!(f, "nil"),
        }
    }
}