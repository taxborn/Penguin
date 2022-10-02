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
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => {
            println!("[LEXER]: {:?}", e);
        }
    }
}
