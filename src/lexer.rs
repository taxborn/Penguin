use std::fs;
#[derive(Debug)] pub enum LexerError {
    InvalidCharacter(char),
    InvalidIdentifier(String),
    InvalidEscapeSequence(char),
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
pub struct Token { kind: TokenKind,
    literal: String
}

impl Token {
    fn new(kind: TokenKind, literal: String) -> Self {
        Self { kind, literal }
    }
}

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    // TODO: Make this it's own structure to allow for multiple files
    // and line numbers to be tracked
    loc: usize
}

impl Lexer {
    pub fn new(file: String) -> Self {
        let contents = fs::read_to_string(file).unwrap();

        Self { 
            source: contents.chars().collect(), 
            loc: 0
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexerError> {
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

                    self.next(); 

                    if let Some(next) = self.current_char() {
                        if next == '=' {
                            // Remove the previous TokenKind::TypeAssignment
                            tokens.pop();

                            // Add the new TokenKind::UnTypedAssignment
                            tokens.push(Token::new(TokenKind::UnTypedAssignment, ":=".to_string()));

                            // Increment the location
                            self.next();
                        }
                    } else {
                        return Err(LexerError::UnexpectedEOF);
                    };

                    self.next();
                },
                '=' => {
                    tokens.push(Token::new(TokenKind::LetAssignment, current.to_string()));

                    self.next();
                },
                ';' => {
                    tokens.push(Token::new(TokenKind::SemiColon, current.to_string()));

                    self.next();
                },
                '\'' | '"' => {
                    let mut buffer = String::new();

                    self.next();

                    while let Some(c) = self.current_char() {
                        if c == '\'' || c == '"' {
                            break;
                        }

                        if c == '\\' {
                            self.next();
                            self.next();

                            if let Some(c) = self.current_char() {
                                match c {
                                    '"' => buffer.push('"'),
                                    _ => return Err(LexerError::InvalidEscapeSequence(c))
                                }
                                break;
                            } else {
                                return Err(LexerError::UnexpectedEOF);
                            }
                        }

                        if let Some(cur) = self.current_char() {
                            buffer.push(cur);
                        } else {
                            return Err(LexerError::UnexpectedEOF);
                        }

                        self.next();
                    }

                    tokens.push(Token::new(TokenKind::String, buffer));

                    self.next();
                },
                _ if current.is_alphabetic() => {
                    let mut buffer = String::new();

                    while let Some(cur) = self.current_char() {
                        if cur.is_alphanumeric() {
                            buffer.push(cur);

                            self.next();
                        } else {
                            break;
                        }
                    }

                    if let Some(kind) = Lexer::identify(&buffer) {
                        tokens.push(Token::new(kind, buffer));
                    } else {
                        tokens.push(Token::new(TokenKind::Identifier, buffer));
                    }

                    self.next();
                },
                _ if current.is_whitespace() => {
                    self.next();
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
    
    fn next(&mut self) {
        self.loc += 1;
    }

    fn identify(buffer: &String) -> Option<TokenKind> {
        match buffer.as_str() {
            "let" | "var" => Some(TokenKind::String),
            _ => None,
        }
    }
}
