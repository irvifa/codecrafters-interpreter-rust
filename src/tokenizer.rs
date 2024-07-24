use std::io::{self, Write};

#[derive(Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Eof,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TokenType::LeftParen => write!(f, "LEFT_PAREN ( null"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN ) null"),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            TokenType::Comma => write!(f, "COMMA , null"),
            TokenType::Dot => write!(f, "DOT . null"),
            TokenType::Minus => write!(f, "MINUS - null"),
            TokenType::Plus => write!(f, "PLUS + null"),
            TokenType::Semicolon => write!(f, "SEMICOLON ; null"),
            TokenType::Star => write!(f, "STAR * null"),
            TokenType::Bang => write!(f, "BANG ! null"),
            TokenType::BangEqual => write!(f, "BANG_EQUAL != null"),
            TokenType::Equal => write!(f, "EQUAL = null"),
            TokenType::EqualEqual => write!(f, "EQUAL_EQUAL == null"),
            TokenType::Less => write!(f, "LESS < null"),
            TokenType::LessEqual => write!(f, "LESS_EQUAL <= null"),
            TokenType::Greater => write!(f, "GREATER > null"),
            TokenType::GreaterEqual => write!(f, "GREATER_EQUAL >= null"),
            TokenType::Eof => write!(f, "EOF  null"),
        }
    }
}

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<TokenType>,
    start: usize,
    current: usize,
    line: usize,
    pub has_errors: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            has_errors: false,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<TokenType> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(TokenType::Eof);
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            }
            ' ' | '\r' | '\t' => {} // Ignore whitespace
            '\n' => self.line += 1,
            _ => {
                self.report_error(c);
                self.has_errors = true;
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(token_type);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn report_error(&self, c: char) {
        writeln!(
            io::stderr(),
            "[line {}] Error: Unexpected character: {}",
            self.line, c
        )
        .unwrap();
    }
}
