#[derive(Debug)] pub enum LexerError {
    InvalidCharacter(char),
    InvalidIdentifier(String),
    InvalidEscapeSequence(char),
    UnexpectedEOF,
}

#[derive(Clone, Debug, PartialEq)]
enum TokenKind {
    TypeAssignment, // :
    Whitespace,     // \s+
    LetAssignment, // =
    UnTypedAssignment, // :=
    SemiColon, // ;
    Identifier,
    Assign, // let
    String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
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
    pub fn new(contents: String) -> Self {
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
                    //
                    // Increment the location
                    self.next();
                },
                '=' => {
                    // Check if the previous token was a TypeAssignment,
                    // if so, this is an UnTypedAssignment
                    if let Some(last) = tokens.last() {
                        if last.kind == TokenKind::TypeAssignment {
                            // Pop the last token
                            tokens.pop();

                            tokens.push(Token::new(TokenKind::UnTypedAssignment, ":=".to_string()));

                            self.next();
                            continue;
                        }
                    }

                    // Check if the previous two tokens are let and a whitespace
                    // If so, this is a let assignment
                    if tokens.len() >= 2 {
                        let last_two = tokens[tokens.len() - 2..].to_vec();

                        let second_last = last_two[0].clone();
                        let last = last_two[1].clone();

                        // Check if the last two tokens are a let and a whitespace,
                        // if so, this is an UnTypedAssignment
                        if last.kind == TokenKind::Whitespace && second_last.kind == TokenKind::TypeAssignment {
                            // Pop the last two tokens
                            tokens.pop();
                            tokens.pop();

                            // Push an untyped assignment
                            tokens.push(Token::new(TokenKind::UnTypedAssignment, ":=".to_string()));

                            self.next();
                            continue;
                        }
                    }

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

                        buffer.push(c);

                        self.next();
                    }

                    println!("String: {}", buffer);

                    tokens.push(Token::new(TokenKind::String, buffer));

                    self.next();
                },
                _ if current.is_alphabetic() => {
                    let mut buffer = String::new();

                    while let Some(cur) = self.current_char() { if cur.is_alphanumeric() {
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
                },
                _ if current.is_whitespace() => {
                    self.next();

                    while let Some(cur) = self.current_char() {
                        if cur.is_whitespace() {
                            self.next();
                        } else {
                            break;
                        }
                    }

                    tokens.push(Token::new(TokenKind::Whitespace, " ".to_string()));
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
            "let" | "var" => Some(TokenKind::Assign),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assignment() {
        let mut lexer = Lexer::new(":=".to_string());
        let tokens = lexer.lex().unwrap();

        assert_eq!(tokens.len(), 1);

        let token = &tokens[0];

        println!("{:?}", token);

        assert_eq!(token.kind, TokenKind::UnTypedAssignment);
    }

    #[test]
    fn test_spaced_assignment() {
        let mut lexer = Lexer::new(": =".to_string());
        let tokens = lexer.lex().unwrap();

        assert_eq!(tokens.len(), 1);

        let token = &tokens[0];

        assert_eq!(token.kind, TokenKind::UnTypedAssignment);
    }

    #[test]
    fn test_typed_assignment() {
        let mut lexer = Lexer::new(": u32 =".to_string());
        let tokens = lexer.lex().unwrap();

        // TODO: Should this be a TypedAssignment token with a type of u32?
        // E.g TokenKind::TypedAssignment("u32"), or would that be done in the parser?
        let expected = vec![
            Token::new(TokenKind::TypeAssignment, ":".to_string()),
            Token::new(TokenKind::Whitespace, " ".to_string()),
            Token::new(TokenKind::Identifier, "u32".to_string()),
            Token::new(TokenKind::Whitespace, " ".to_string()),
            Token::new(TokenKind::LetAssignment, "=".to_string())
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_variable_assignment_to_string() {
        let mut lexer = Lexer::new("let x : = 'hello world';".to_string());
        let tokens = lexer.lex().unwrap();

        let expected_tokens = vec![Token::new(TokenKind::Assign, "let".to_string()),
                                   Token::new(TokenKind::Whitespace, " ".to_string()),
                                   Token::new(TokenKind::Identifier, "x".to_string()),
                                   Token::new(TokenKind::Whitespace, " ".to_string()),
                                   Token::new(TokenKind::UnTypedAssignment, ":=".to_string()),
                                   Token::new(TokenKind::Whitespace, " ".to_string()),
                                   Token::new(TokenKind::String, "hello world".to_string()),
                                   Token::new(TokenKind::SemiColon, ";".to_string())];

        assert_eq!(tokens, expected_tokens);
    }
}
