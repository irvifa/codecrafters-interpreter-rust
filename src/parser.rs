use crate::ast::{Expr, LiteralValue};
use crate::tokenizer::{TokenType, Scanner};

pub struct Parser<'a> {
    scanner: Scanner<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Parser {
            scanner: Scanner::new(source),
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.literal()
    }

    fn literal(&mut self) -> Result<Expr, String> {
        let tokens = self.scanner.scan_tokens();
        if tokens.is_empty() {
            return Err("Unexpected end of input".to_string());
        }

        match &tokens[0].0 {
            TokenType::True => Ok(Expr::Literal(LiteralValue::Bool(true))),
            TokenType::False => Ok(Expr::Literal(LiteralValue::Bool(false))),
            TokenType::Nil => Ok(Expr::Literal(LiteralValue::Nil)),
            TokenType::Number => {
                let value = tokens[0].2.parse::<f64>().unwrap();
                Ok(Expr::Literal(LiteralValue::Number(value)))
            }
            TokenType::String => {
                let value = tokens[0].2.clone();
                Ok(Expr::Literal(LiteralValue::String(value)))
            }
            _ => Err(format!("Unexpected token: {:?}", tokens[0])),
        }
    }
}
