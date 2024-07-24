#[derive(Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Eof,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TokenType::LeftParen => write!(f, "LEFT_PAREN ( null"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN ) null"),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            TokenType::Eof => write!(f, "EOF  null"),
        }
    }
}

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<TokenType> {
        let chars: Vec<char> = self.source.chars().collect();

        for &c in &chars {
            match c {
                '(' => self.tokens.push(TokenType::LeftParen),
                ')' => self.tokens.push(TokenType::RightParen),
                '{' => self.tokens.push(TokenType::LeftBrace),
                '}' => self.tokens.push(TokenType::RightBrace),
                _ => {}
            }
        }

        self.tokens.push(TokenType::Eof);
        &self.tokens
    }
}
