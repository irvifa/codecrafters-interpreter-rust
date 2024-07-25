use std::io::{self, Write};

#[derive(Debug, PartialEq)]
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
    Slash,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    String,
    Number,
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
            TokenType::Slash => write!(f, "SLASH / null"),
            TokenType::Bang => write!(f, "BANG ! null"),
            TokenType::BangEqual => write!(f, "BANG_EQUAL != null"),
            TokenType::Equal => write!(f, "EQUAL = null"),
            TokenType::EqualEqual => write!(f, "EQUAL_EQUAL == null"),
            TokenType::Less => write!(f, "LESS < null"),
            TokenType::LessEqual => write!(f, "LESS_EQUAL <= null"),
            TokenType::Greater => write!(f, "GREATER > null"),
            TokenType::GreaterEqual => write!(f, "GREATER_EQUAL >= null"),
            TokenType::String => write!(f, "STRING"),
            TokenType::Number => write!(f, "NUMBER"),
            TokenType::Eof => write!(f, "EOF  null"),
        }
    }
}

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<(TokenType, String, String)>,
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

    pub fn scan_tokens(&mut self) -> &Vec<(TokenType, String, String)> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
    
        // Only add EOF token if all tokens have been processed
        if let Some(&(ref last_type, _, _)) = self.tokens.last() {
            if *last_type != TokenType::Eof {
                self.tokens.push((TokenType::Eof, String::new(), String::new()));
            }
        } else {
            self.tokens.push((TokenType::Eof, String::new(), String::new()));
        }
    
        &self.tokens
    }    

    fn scan_token(&mut self) {
        let c = self.advance().unwrap_or('\0');
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
            '/' => {
                if self.match_char('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
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
            '"' => self.string(),
            ' ' | '\r' | '\t' => {} // Ignore whitespace
            '\n' => self.line += 1,
            '0'..='9' => self.number(),
            _ => {
                    self.report_error(c);
                    self.has_errors = true;
                
            }
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            let c = self.source.chars().nth(self.current);
            self.current += 1;
            c
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.current += 1;
            return true;
        }
        false
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            self.source.chars().nth(self.current)
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            None
        } else {
            self.source.chars().nth(self.current + 1)
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push((token_type, text.clone(), text));
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, lexeme: String, literal: String) {
        self.tokens.push((token_type, lexeme, literal));
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

    fn string(&mut self) {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            writeln!(io::stderr(), "[line {}] Error: Unterminated string.", self.line).unwrap();
            self.has_errors = true;
            return;
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = self.source[(self.start + 1)..(self.current - 1)].to_string();
        self.add_token_with_literal(TokenType::String, format!("\"{}\"", value), value);
    }

    fn number(&mut self) {
        while self.peek().map_or(false, |c| c.is_digit(10)) {
            self.advance();
        }
    
        // Look for a fractional part.
        if self.peek() == Some('.') && self.peek_next().map_or(false, |c| c.is_digit(10)) {
            // Consume the "."
            self.advance();
    
            while self.peek().map_or(false, |c| c.is_digit(10)) {
                self.advance();
            }
        }
    
        let value = &self.source[self.start..self.current];
        let lexeme = value.to_string();
        let literal = if value.contains('.') {
            value.to_string()
        } else {
            format!("{}.0", value)
        };
    
        self.add_token_with_literal(TokenType::Number, lexeme, literal);
    }
}
