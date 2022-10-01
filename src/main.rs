#![allow(dead_code)]

use std::{ env, fs };

#[derive(Debug)]
enum LexerError {
    InvalidCharacter(char),
    UnexpectedEOF,
}

#[derive(Debug)]
enum TokenKind {
    TypeAssignment, // :
    LetAssignment, // =
    UnTypedAssignment, // :=
    SemiColon, // ;
    Identifier,
    Assign, // let
    String,
}

#[derive(Debug)]
struct Token { kind: TokenKind,
    literal: String
}

impl Token {
    fn new(kind: TokenKind, literal: String) -> Self {
        Self { kind, literal }
    }
}

#[derive(Debug)]
struct Lexer {
    source: Vec<char>,
    loc: usize
}

impl Lexer {
    fn new(file: String) -> Self {
        let contents = fs::read_to_string(file).unwrap();

        Self { 
            source: contents.chars().collect(), 
            loc: 0
        }
    }

    fn lex(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = vec![];

        // While we are not at the end of the contents
        while self.source.len() > self.loc {
            let current = if let Some(c) = self.current_char() {
                c
            } else {
                // Reached the end of the file
                break;
            };

            match current {
                ':' => {
                    tokens.push(Token::new(TokenKind::TypeAssignment, current.to_string()));

                    self.loc += 1;

                    if let Some(next) = self.current_char() {
                        if next == '=' {
                            // Remove the previous TokenKind::TypeAssignment
                            tokens.pop();

                            // Add the new TokenKind::UnTypedAssignment
                            tokens.push(Token::new(TokenKind::UnTypedAssignment, ":=".to_string()));

                            // Increment the location
                            self.loc += 1;
                        }
                    } else {
                        return Err(LexerError::UnexpectedEOF);
                    };

                    self.loc += 1;
                },
                '=' => {
                    tokens.push(Token::new(TokenKind::LetAssignment, current.to_string()));

                    self.loc += 1;
                },
                ';' => {
                    tokens.push(Token::new(TokenKind::SemiColon, current.to_string()));

                    self.loc += 1;
                },
                '\'' | '"' => {
                    let mut buffer = String::new();

                    self.loc += 1;

                    while let Some(c) = self.current_char() {
                        if c == '\'' || c == '"' {
                            break;
                        }

                        buffer.push(c);

                        self.loc += 1;
                    }

                    tokens.push(Token::new(TokenKind::String, buffer));

                    self.loc += 1;
                },
                _ if current.is_alphabetic() => {
                    let mut buffer = String::new();

                    while let Some(c) = self.current_char() {
                        if c.is_alphanumeric() {
                            buffer.push(c);

                            self.loc += 1;
                        } else {
                            break;
                        }
                    }

                    if buffer == "let" || buffer == "var" {
                        tokens.push(Token::new(TokenKind::Assign, buffer));
                    } else {
                        tokens.push(Token::new(TokenKind::Identifier, buffer));
                    }

                    self.loc += 1;
                },
                _ if current.is_whitespace() => {
                    self.loc += 1;
                },
                _ => {
                    return Err(LexerError::InvalidCharacter(current));
                }
            }
        }

        Ok(tokens)
    }

    fn current_char(&self) -> Option<char> {
        self.source.get(self.loc).cloned()
    }
}

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
