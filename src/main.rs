#![allow(dead_code)]
use lexer::Lexer;
use std::{env, fs};

mod lexer;

fn main() {
    let file = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(&file).unwrap();

    let mut lexer = Lexer::new(contents);

    match lexer.lex() {
        Ok(tokens) => {
            println!("Tokens: {:#?}", tokens);
        }
        Err(e) => {
            println!("[LEXER ERROR]: {}", e);
        }
    }
}
