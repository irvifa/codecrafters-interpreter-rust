use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let mut scanner = Scanner::new(&file_contents);
            let tokens = scanner.scan_tokens();

            for token in tokens {
                println!("{}", token);
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
        }
    }
}

#[derive(Debug)]
enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    EOF,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TokenType::LEFT_PAREN => write!(f, "LEFT_PAREN ( null"),
            TokenType::RIGHT_PAREN => write!(f, "RIGHT_PAREN ) null"),
            TokenType::EOF => write!(f, "EOF  null"),
        }
    }
}

struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<TokenType>,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
        }
    }

    fn scan_tokens(&mut self) -> &Vec<TokenType> {
        let chars: Vec<char> = self.source.chars().collect();

        for &c in &chars {
            match c {
                '(' => self.tokens.push(TokenType::LEFT_PAREN),
                ')' => self.tokens.push(TokenType::RIGHT_PAREN),
                _ => {}
            }
        }

        self.tokens.push(TokenType::EOF);
        &self.tokens
    }
}
