#![allow(dead_code)]

use std::env;

mod lexer;

use lexer::Lexer;

fn main() {
    let file = env::args().nth(1).unwrap();

    let mut lexer = Lexer::new(file);

    match lexer.lex() {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
