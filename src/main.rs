#![allow(dead_code)]
use clap::Parser;
use lexer::Lexer;
use std::fs;
use std::path::PathBuf;
use std::time;

mod lexer;

#[derive(Parser, Debug)]
#[command(name = "Penguin Compiler")]
#[command(about = "A compiler for the Penguin programming language")]
#[command(version)]
struct Args {
    /// The input file to compile
    #[arg(short, long, value_name = "source.pg")]
    file: PathBuf,

    /// Print the tokens
    #[arg(long)]
    tokens: bool,

    /// Time the compilation
    #[arg(long)]
    time: bool,
}

fn main() {
    let args = Args::parse();
    let file = args.file;

    let contents = fs::read_to_string(&file).unwrap();
    let mut lexer = Lexer::new(contents);
    let start = time::Instant::now();
    let tokens = lexer.lex();
    let end = start.elapsed();

    match tokens {
        Ok(tokens) => {
            if args.tokens {
                println!("Tokens: {:#?}", tokens);
            }

            if args.time {
                let chars_per_second = ((lexer.loc.index as f64) / (end.as_secs_f64())) as usize;
                println!(
                    "Time: {:?} microseconds, or a speed of {} chars per second.",
                    end.as_micros(),
                    chars_per_second
                );
            }

            println!("Sucessfully compiled.");
        }
        Err(error) => {
            println!("[LEXER ERROR]: {}", error);
        }
    }
}
