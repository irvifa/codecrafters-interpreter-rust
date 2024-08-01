
mod tokenizer;
mod parser;
mod ast;

use std::env;
use std::fs;
use std::io::{self, Write};
use crate::tokenizer::{Scanner, TokenType};
use crate::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} <command> <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    match command.as_str() {
        "tokenize" => {
            let mut scanner = Scanner::new(&file_contents);
            let tokens = scanner.scan_tokens();

            for (token, lexeme, literal) in tokens {
                match token {
                    TokenType::String => println!("STRING {} {}", lexeme, literal),
                    TokenType::Number => println!("NUMBER {} {}", lexeme, literal),
                    TokenType::Identifier => println!("IDENTIFIER {} null", lexeme),
                    TokenType::Eof => println!("EOF  null"),
                    _ => println!("{}", token),
                }
            }

            if scanner.has_errors {
                std::process::exit(65);
            }
        },
        "parse" => {
            let mut parser = Parser::new(&file_contents);
            match parser.parse() {
                Ok(expr) => {
                    println!("{}", expr);
                },
                Err(e) => {
                    writeln!(io::stderr(), "Error: {}", e).unwrap();
                    std::process::exit(65);
                }
            }
        },
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
        }
    }
}