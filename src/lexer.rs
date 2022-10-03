#[derive(Debug)]
pub enum LexerError {
    InvalidCharacter(char),
    InvalidIdentifier(String),
    InvalidEscapeSequence(char),
    UnexpectedEOF,
}

#[derive(Clone, Debug, PartialEq)]
enum TokenKind {
    TypeAssignment,          // :
    TypedAssignment(String), // : u32 =
    LetAssignment,           // =
    UnTypedAssignment,       // :=
    SemiColon,               // ;
    Identifier,
    Assign, // let
    String,

    Number(usize),

    // Arithmetic
    Plus,           // +
    ShortIncrement, // +=

    Minus,          // -
    ShortDecrement, // -=

    Multiply,      // *
    ShortMultiply, // *=

    Divide,      // /
    ShortDivide, // /=

    Modulo,      // %
    ShortModulo, // %=
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    // The kind of token
    kind: TokenKind,

    // The characters that were used to create this token. This should be
    // unchanged from the original source code.
    literal: String,
}

impl Token {
    // Create a new token
    fn new(kind: TokenKind, literal: String) -> Self {
        Self { kind, literal }
    }
}

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    // TODO: Make this it's own structure to allow for multiple files
    // and line numbers to be tracked
    loc: usize,
}

impl Lexer {
    pub fn new(contents: String) -> Self {
        Self {
            source: contents.chars().collect(),
            loc: 0,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = vec![];

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

                    // Increment the location
                    self.next();
                }
                '=' => {
                    // Check if the previous token was a TypeAssignment,
                    // if so, this is an UnTypedAssignment
                    if let Some(last) = tokens.last() {
                        match last.kind {
                            TokenKind::TypeAssignment => {
                                // Pop the last token
                                tokens.pop();

                                tokens.push(Token::new(
                                    TokenKind::UnTypedAssignment,
                                    ":=".to_string(),
                                ));
                            }
                            TokenKind::Plus => {
                                // Pop the last token
                                tokens.pop();

                                tokens
                                    .push(Token::new(TokenKind::ShortIncrement, "+=".to_string()));
                            }
                            TokenKind::Minus => {
                                // Pop the last token
                                tokens.pop();

                                tokens
                                    .push(Token::new(TokenKind::ShortDecrement, "-=".to_string()));
                            }
                            TokenKind::Multiply => {
                                // Pop the last token
                                tokens.pop();

                                tokens.push(Token::new(TokenKind::ShortMultiply, "*=".to_string()));
                            }
                            TokenKind::Divide => {
                                // Pop the last token
                                tokens.pop();

                                tokens.push(Token::new(TokenKind::ShortDivide, "/=".to_string()));
                            }
                            TokenKind::Modulo => {
                                // Pop the last token
                                tokens.pop();

                                tokens.push(Token::new(TokenKind::ShortModulo, "%=".to_string()));
                            }
                            _ => {
                                tokens.push(Token::new(
                                    TokenKind::LetAssignment,
                                    current.to_string(),
                                ));
                            }
                        }
                    }

                    self.next();
                }
                ';' => {
                    tokens.push(Token::new(TokenKind::SemiColon, current.to_string()));

                    self.next();
                }
                '\'' | '"' => {
                    let mut found_close = false;
                    let mut buffer = String::new();

                    self.next();

                    while let Some(c) = self.current_char() {
                        // Check if the current string quote is the same as the
                        // starting quote, if so, we have found the end of the
                        // string.
                        if c == current {
                            found_close = true;

                            break;
                        }

                        // Check if the current character is an escape sequence
                        if c == '\\' {
                            self.next();

                            // Match the type of escape sequence
                            if let Some(c) = self.current_char() {
                                match c {
                                    'n' => buffer.push('\n'),
                                    't' => buffer.push('\t'),
                                    'r' => buffer.push('\r'),
                                    '0' => buffer.push('\0'),
                                    '"' => buffer.push('"'),
                                    '\\' => buffer.push('\\'),
                                    '\'' => buffer.push('\''),
                                    _ => return Err(LexerError::InvalidEscapeSequence(c)),
                                }
                            }
                        } else {
                            buffer.push(c);
                        }

                        self.next();
                    }

                    // Check if we found the closing quote
                    if !found_close {
                        return Err(LexerError::UnexpectedEOF);
                    }

                    tokens.push(Token::new(TokenKind::String, buffer));

                    self.next();
                }
                // Identifiers start with a letter (underscore in the future)
                // and can contain numbers.
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
                }
                // TODO: Add support for floats
                _ if current.is_numeric() => {
                    let mut buffer = String::new();

                    while let Some(cur) = self.current_char() {
                        // Check if the current character is a number or an
                        // underscore. Underscores are used to make numbers
                        // more readable, for example, 1_000_000.
                        if cur.is_numeric() || cur == '_' {
                            buffer.push(cur);

                            self.next();
                        } else {
                            break;
                        }
                    }

                    let num = buffer.replace("_", "").parse::<usize>().unwrap();

                    tokens.push(Token::new(TokenKind::Number(num), buffer));
                }
                '+' => {
                    tokens.push(Token::new(TokenKind::Plus, current.to_string()));

                    self.next();
                }
                '-' => {
                    tokens.push(Token::new(TokenKind::Minus, current.to_string()));

                    self.next();
                }
                '*' => {
                    tokens.push(Token::new(TokenKind::Multiply, current.to_string()));
                    self.next();
                }
                '/' => {
                    tokens.push(Token::new(TokenKind::Divide, current.to_string()));

                    self.next();
                }
                '%' => {
                    tokens.push(Token::new(TokenKind::Modulo, current.to_string()));

                    self.next();
                }
                _ if current.is_whitespace() => {
                    // TODO: Should we include whitespace tokens?
                    self.next();
                }
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

    fn identify(buffer: &str) -> Option<TokenKind> {
        // Change the buffer to lowercase to make it easier to compare
        let buffer = buffer.to_lowercase();

        match buffer.as_str() {
            "let" => Some(TokenKind::Assign),
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

        let expected = vec![Token::new(TokenKind::UnTypedAssignment, ":=".to_string())];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_spaced_assignment() {
        let mut lexer = Lexer::new(": =".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![Token::new(TokenKind::UnTypedAssignment, ":=".to_string())];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_typed_assignment() {
        let mut lexer = Lexer::new(": u32 =".to_string());
        let tokens = lexer.lex().unwrap();

        // TODO: Should this be a TypedAssignment token with a type of u32?
        // E.g TokenKind::TypedAssignment("u32"), or would that be done in the parser?
        let expected = vec![
            Token::new(TokenKind::TypeAssignment, ":".to_string()),
            Token::new(TokenKind::Identifier, "u32".to_string()),
            Token::new(TokenKind::LetAssignment, "=".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_variable_assignment_to_string() {
        let mut lexer = Lexer::new("let x : = \"hello world\";".to_string());
        let tokens = lexer.lex().unwrap();

        let expected_tokens = vec![
            Token::new(TokenKind::Assign, "let".to_string()),
            Token::new(TokenKind::Identifier, "x".to_string()),
            Token::new(TokenKind::UnTypedAssignment, ":=".to_string()),
            Token::new(TokenKind::String, "hello world".to_string()),
            Token::new(TokenKind::SemiColon, ";".to_string()),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_escape_sequences() {
        let mut lexer = Lexer::new("'Don\\'t'".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![Token::new(TokenKind::String, "Don't".to_string())];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_string_with_escaped_quotes() {
        let mut lexer = Lexer::new("\"\\\"hello\\\"\"".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![Token::new(TokenKind::String, "\"hello\"".to_string())];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_string_with_mixed_quotes() {
        let mut lexer = Lexer::new("\"Hello, 'world!'\"".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![Token::new(TokenKind::String, "Hello, 'world!'".to_string())];

        assert_eq!(tokens, expected);

        let mut lexer_flipped = Lexer::new("'Hello, \"world!\"'".to_string());
        let tokens_flipped = lexer_flipped.lex().unwrap();

        let expected_flipped = vec![Token::new(
            TokenKind::String,
            "Hello, \"world!\"".to_string(),
        )];

        assert_eq!(tokens_flipped, expected_flipped);
    }

    #[test]
    fn test_string_with_escaped_backslash() {
        let mut lexer = Lexer::new("\"\\\\hello\\\\\"".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![Token::new(TokenKind::String, "\\hello\\".to_string())];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_number() {
        let mut lexer = Lexer::new("123".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![Token::new(TokenKind::Number(123), "123".to_string())];

        assert_eq!(tokens, expected);

        let number = match tokens[0].kind {
            TokenKind::Number(n) => n,
            _ => panic!("Expected a number token"),
        };

        assert_eq!(number, 123);
    }

    #[test]
    fn test_number_with_seperator() {
        let mut lexer = Lexer::new("1_000".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![Token::new(TokenKind::Number(1000), "1_000".to_string())];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_short_increment() {
        let mut lexer = Lexer::new("x += 5;".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![
            Token::new(TokenKind::Identifier, "x".to_string()),
            Token::new(TokenKind::ShortIncrement, "+=".to_string()),
            Token::new(TokenKind::Number(5), "5".to_string()),
            Token::new(TokenKind::SemiColon, ";".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_short_decrement() {
        let mut lexer = Lexer::new("x -= 5;".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![
            Token::new(TokenKind::Identifier, "x".to_string()),
            Token::new(TokenKind::ShortDecrement, "-=".to_string()),
            Token::new(TokenKind::Number(5), "5".to_string()),
            Token::new(TokenKind::SemiColon, ";".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_case_insensitive_keywords() {
        let mut lexer = Lexer::new("LET x : = 123;".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![
            Token::new(TokenKind::Assign, "LET".to_string()),
            Token::new(TokenKind::Identifier, "x".to_string()),
            Token::new(TokenKind::UnTypedAssignment, ":=".to_string()),
            Token::new(TokenKind::Number(123), "123".to_string()),
            Token::new(TokenKind::SemiColon, ";".to_string()),
        ];

        assert_eq!(tokens, expected);
    }
}
