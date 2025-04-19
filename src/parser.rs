// parser.rs
use crate::ast::{Expr, LiteralValue};
use crate::tokenizer::{Scanner, TokenType};

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    current: usize,
    tokens: Vec<(TokenType, String, String)>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().to_vec();
        Parser {
            scanner,
            current: 0,
            tokens,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.term()
    }

    // term -> factor ( ( "+" | "-" ) factor )*
    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().0.clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().0.clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().0.clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        if self.is_at_end() {
            return Err("Unexpected end of input".to_string());
        }

        match &self.peek().0 {
            TokenType::True => {
                self.advance();
                Ok(Expr::Literal(LiteralValue::Bool(true)))
            }
            TokenType::False => {
                self.advance();
                Ok(Expr::Literal(LiteralValue::Bool(false)))
            }
            TokenType::Nil => {
                self.advance();
                Ok(Expr::Literal(LiteralValue::Nil))
            }
            TokenType::Number => {
                let value = self.advance().2.parse::<f64>().map_err(|e| e.to_string())?;
                Ok(Expr::Literal(LiteralValue::Number(value)))
            }
            TokenType::String => {
                let value = self.advance().2.clone();
                Ok(Expr::Literal(LiteralValue::String(value)))
            }
            _ => Err(format!("Unexpected token: {:?}", self.peek())),
        }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(
        &mut self,
        t: TokenType,
        message: &str,
    ) -> Result<&(TokenType, String, String), String> {
        if self.check(&t) {
            Ok(self.advance())
        } else {
            Err(message.to_string())
        }
    }

    fn check(&self, t: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().0 == t
        }
    }

    fn advance(&mut self) -> &(TokenType, String, String) {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().0 == TokenType::Eof
    }

    fn peek(&self) -> &(TokenType, String, String) {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &(TokenType, String, String) {
        &self.tokens[self.current - 1]
    }
}
