#![allow(dead_code)]
use clap::Parser;
use lexer::Lexer;
use std::fs;
use std::path::PathBuf;

mod lexer;

#[derive(Parser, Debug)]
#[command(name = "Penguin Compiler")]
#[command(about = "A compiler for the Penguin programming language", long_about = None)]
#[command(version = "0.0.1")]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    file: PathBuf,
}

fn main() {
    let args = Args::parse();
    let file = args.file;

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
