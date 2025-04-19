// interpreter.rs
use crate::ast::{Expr, LiteralValue};

pub struct Interpreter;

impl Interpreter {
    pub fn evaluate(&self, expr: Expr) -> LiteralValue {
        match expr {
            Expr::Literal(val) => val,
            Expr::Grouping(inner) => self.evaluate(*inner),
            Expr::Unary(op, right) => {
                let right_val = self.evaluate(*right);
                match op {
                    crate::tokenizer::TokenType::Minus => {
                        if let LiteralValue::Number(n) = right_val {
                            LiteralValue::Number(-n)
                        } else {
                            panic!("Operand must be a number")
                        }
                    }
                    crate::tokenizer::TokenType::Bang => LiteralValue::Bool(!is_truthy(right_val)),
                    _ => panic!("Unknown unary operator"),
                }
            }
            Expr::Binary(left, op, right) => {
                let left_val = self.evaluate(*left);
                let right_val = self.evaluate(*right);

                use crate::tokenizer::TokenType::*;
                match op {
                    Plus => match (left_val, right_val) {
                        (LiteralValue::Number(a), LiteralValue::Number(b)) => {
                            LiteralValue::Number(a + b)
                        }
                        (LiteralValue::String(a), LiteralValue::String(b)) => {
                            LiteralValue::String(a + &b)
                        }
                        _ => panic!("Operands must be two numbers or two strings"),
                    },
                    Minus => bin_op_numeric(left_val, right_val, |a, b| a - b),
                    Star => bin_op_numeric(left_val, right_val, |a, b| a * b),
                    Slash => bin_op_numeric(left_val, right_val, |a, b| a / b),
                    EqualEqual => LiteralValue::Bool(left_val == right_val),
                    BangEqual => LiteralValue::Bool(left_val != right_val),
                    Greater => bin_op_numeric_bool(left_val, right_val, |a, b| a > b),
                    GreaterEqual => bin_op_numeric_bool(left_val, right_val, |a, b| a >= b),
                    Less => bin_op_numeric_bool(left_val, right_val, |a, b| a < b),
                    LessEqual => bin_op_numeric_bool(left_val, right_val, |a, b| a <= b),
                    _ => panic!("Unknown binary operator"),
                }
            }
        }
    }
}

fn is_truthy(val: LiteralValue) -> bool {
    match val {
        LiteralValue::Nil => false,
        LiteralValue::Bool(b) => b,
        _ => true,
    }
}

fn bin_op_numeric<F: Fn(f64, f64) -> f64>(a: LiteralValue, b: LiteralValue, op: F) -> LiteralValue {
    if let (LiteralValue::Number(a), LiteralValue::Number(b)) = (a, b) {
        LiteralValue::Number(op(a, b))
    } else {
        panic!("Operands must be numbers")
    }
}

fn bin_op_numeric_bool<F: Fn(f64, f64) -> bool>(
    a: LiteralValue,
    b: LiteralValue,
    op: F,
) -> LiteralValue {
    if let (LiteralValue::Number(a), LiteralValue::Number(b)) = (a, b) {
        LiteralValue::Bool(op(a, b))
    } else {
        panic!("Operands must be numbers")
    }
}
