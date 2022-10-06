#[derive(Debug)]
/// Errors that can occur during lexing.
pub enum LexerError {
    /// An invalid character was encountered.
    InvalidCharacter(char),
    /// An invalid identifier was encountered.
    InvalidIdentifier(String),
    /// An invalid escape sequence was encountered.
    InvalidEscapeSequence(char),
    /// Unexpected end of input.
    UnexpectedEOF,
}

#[derive(Clone, Debug, PartialEq)]
/// A token is a single lexical unit of the language.
pub enum TokenKind {
    /// A semicolon (:), typically followed by a type or equal sign
    TypeAssignment, // :
    /// An equal sign, typically preceded by a type or TypeAssignment
    LetAssignment, // =
    /// An assignment that contains a type
    ///
    /// E.g. `let a : u32 = 10;`
    TypedAssignment(String), // : u32 =
    /// An assignment that does not contain a type
    ///
    /// E.g. `let a := "Waddle";`
    UnTypedAssignment, // :=
    /// A Semicolon
    Semicolon, // ;
    /// Any string of characters that are not symbols in the language
    ///
    /// E.g. `let` is an identifier.
    Identifier,
    /// `let`
    Assign, // let
    /// Any single (') or double (") quoted strings, allows for escape sequences
    String,

    /// A number
    Number(usize),

    // Arithmetic
    /// Addition (+)
    Plus, // +
    /// Addition assignment (+=)
    ShortIncrement, // +=

    /// Subtraction (-)
    Minus, // -
    /// Subtraction assignment (-=)
    ShortDecrement, // -=

    /// Multiplication (*)
    Multiply, // *
    /// Multiplication assignment (*=)
    ShortMultiply, // *=

    /// Division (/)
    Divide, // /
    /// Division assignment (/=)
    ShortDivide, // /=

    /// Modulo (%)
    Modulo, // %
    /// Modulo assignment (%=)
    ShortModulo, // %=

    /// Open parenthesis
    OpenParen, // (
    /// Close parenthesis
    CloseParen, // )

    /// Open curly brace
    OpenBrace, // {
    /// Close curly brace
    CloseBrace, // }

    /// Open square bracket
    OpenBracket, // [
    /// Close square bracket
    CloseBracket, // ]

    /// A comma
    Comma, // ,

    /// Function
    Function, // func

    /// Return
    Return, // return

    /// Import
    Import, // import
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    // The kind of token
    pub kind: TokenKind,

    // The characters that were used to create this token. This should be
    // unchanged from the original source code.
    pub literal: String,
}

impl Token {
    /// Create a new token.
    pub fn new(kind: TokenKind, literal: String) -> Self {
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
    /// Create a new lexer from a string.
    pub fn new(contents: String) -> Self {
        Self {
            source: contents.chars().collect(),
            loc: 0,
        }
    }

    /// Lex the source code into a list of tokens.
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
                    if let Some(last) = tokens.last() {
                        match last.kind {
                            // If the last token was a type assignment, then this is an untyped assignment
                            TokenKind::TypeAssignment => {
                                // Pop the last token
                                tokens.pop();

                                tokens.push(Token::new(
                                    TokenKind::UnTypedAssignment,
                                    ":=".to_string(),
                                ));
                            }
                            // If the last token was a plus, then this is a short increment
                            TokenKind::Plus => {
                                // Pop the last token
                                tokens.pop();

                                tokens
                                    .push(Token::new(TokenKind::ShortIncrement, "+=".to_string()));
                            }
                            // If the last token was a minus, then this is a short decrement
                            TokenKind::Minus => {
                                // Pop the last token
                                tokens.pop();

                                tokens
                                    .push(Token::new(TokenKind::ShortDecrement, "-=".to_string()));
                            }
                            // If the last token was a multiply, then this is a short multiply
                            TokenKind::Multiply => {
                                // Pop the last token
                                tokens.pop();

                                tokens.push(Token::new(TokenKind::ShortMultiply, "*=".to_string()));
                            }
                            // If the last token was a modulo, then this is a short modulo
                            TokenKind::Modulo => {
                                // Pop the last token
                                tokens.pop();

                                tokens.push(Token::new(TokenKind::ShortModulo, "%=".to_string()));
                            }
                            // Otherwise, this is a normal assignment
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
                    tokens.push(Token::new(TokenKind::Semicolon, current.to_string()));

                    self.next();
                }
                '\'' | '"' => {
                    let mut found_close = false;
                    let mut buffer = String::new();

                    self.next();

                    while let Some(next) = self.current_char() {
                        // Check if the current string quote is the same as the
                        // starting quote, if so, we have found the end of the
                        // string.
                        if next == current {
                            found_close = true;

                            break;
                        }

                        // Check if the current character is an escape sequence
                        // otherwise, just add it to the buffer
                        if next == '\\' {
                            self.next();

                            // Match the type of escape sequence
                            if let Some(next) = self.current_char() {
                                match next {
                                    'n' => buffer.push('\n'),
                                    't' => buffer.push('\t'),
                                    'r' => buffer.push('\r'),
                                    '0' => buffer.push('\0'),
                                    '"' => buffer.push('"'),
                                    '\'' => buffer.push('\''),
                                    '\\' => buffer.push('\\'),
                                    // Ignore new lines, just continue. I
                                    // actually don't know if this is the
                                    // correct way to handle this, but it
                                    // works for now.
                                    '\n' => self.next(),
                                    _ => return Err(LexerError::InvalidEscapeSequence(next)),
                                }
                            }
                        } else {
                            buffer.push(next);
                        }

                        self.next();
                    }

                    // If we didn't find the end of the string, return an error
                    // TODO: Add line numbers
                    if !found_close {
                        return Err(LexerError::UnexpectedEOF);
                    }

                    tokens.push(Token::new(TokenKind::String, buffer));

                    self.next();
                }
                // Identifiers start with a letter (underscore in the future)
                // and can contain numbers.
                '_' | 'a'..='z' | 'A'..='Z' => {
                    let mut buffer = String::new();

                    while let Some(cur) = self.current_char() {
                        if cur.is_alphanumeric() || cur == '_' {
                            buffer.push(cur);

                            self.next();
                        } else {
                            break;
                        }
                    }

                    // Check if the buffer is a keyword, otherwise, it is an
                    // identifier
                    let token = Lexer::identify(&buffer);

                    tokens.push(token);
                }
                // TODO: Add support for floats
                _ if current.is_numeric() => {
                    let mut buffer = String::new();

                    buffer.push(current);

                    self.next();

                    while let Some(next) = self.current_char() {
                        // Check if the current character is a number or an
                        // underscore. Underscores are used to make numbers
                        // more readable, for example, 1_000_000.
                        if next.is_numeric() || next == '_' {
                            buffer.push(next);

                            self.next();
                        } else {
                            break;
                        }
                    }

                    // Strip the underscores from the number, then parse it
                    let num = buffer.replace('_', "").parse::<usize>().unwrap();

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
                '%' => {
                    tokens.push(Token::new(TokenKind::Modulo, current.to_string()));

                    self.next();
                }
                '/' => {
                    self.next();

                    if let Some(next) = self.current_char() {
                        if next == '/' {
                            // This is a comment, skip until the end of the line
                            while let Some(next) = self.current_char() {
                                if next == '\n' {
                                    break;
                                }

                                self.next();
                            }
                        } else if next == '*' {
                            // This is a multi-line comment, skip until the end
                            let mut found_close = false;

                            // Check if there is a closing comment tag,
                            // if so, break out of the loop.
                            //
                            // TODO: Do we want to check for a closing
                            // comment tag? Or allow the user to forget
                            // to close the comment?
                            while let Some(next) = self.current_char() {
                                if next == '*' {
                                    self.next();

                                    if let Some(next) = self.current_char() {
                                        if next == '/' {
                                            found_close = true;
                                            break;
                                        }
                                    }
                                }

                                self.next();
                            }

                            if !found_close {
                                return Err(LexerError::UnexpectedEOF);
                            }
                        } else if next == '=' {
                            tokens.push(Token::new(TokenKind::ShortDivide, "/=".to_string()));
                        } else {
                            // This is a division, push the token and move back
                            // because we probably need to check what it was
                            // dividing by.
                            tokens.push(Token::new(TokenKind::Divide, current.to_string()));

                            self.prev();
                        }
                    }

                    self.next();
                }
                '(' => {
                    tokens.push(Token::new(TokenKind::OpenParen, current.to_string()));

                    self.next();
                }
                ')' => {
                    tokens.push(Token::new(TokenKind::CloseParen, current.to_string()));

                    self.next();
                }
                '{' => {
                    tokens.push(Token::new(TokenKind::OpenBrace, current.to_string()));

                    self.next();
                }
                '}' => {
                    tokens.push(Token::new(TokenKind::CloseBrace, current.to_string()));

                    self.next();
                }
                '[' => {
                    tokens.push(Token::new(TokenKind::OpenBracket, current.to_string()));

                    self.next();
                }
                ']' => {
                    tokens.push(Token::new(TokenKind::CloseBracket, current.to_string()));

                    self.next();
                }
                ',' => {
                    tokens.push(Token::new(TokenKind::Comma, current.to_string()));

                    self.next();
                }
                _ if current.is_whitespace() => {
                    // TODO: Should we include whitespace tokens?
                    // For now, we will ignore them
                    self.next();
                }
                _ => {
                    return Err(LexerError::InvalidCharacter(current));
                }
            }
        }

        Ok(tokens)
    }

    /// Get the current character in the source
    fn current_char(&self) -> Option<char> {
        self.source.get(self.loc).cloned()
    }

    /// Move the lexer to the next character
    fn next(&mut self) {
        self.loc += 1;
    }

    /// Move the lexer to the previous character
    fn prev(&mut self) {
        self.loc -= 1;
    }

    /// Identify a keyword based on a buffer
    ///
    /// # Arguments
    /// * `buffer` - The buffer to identify
    ///
    /// # Returns
    /// The keyword if it exists, otherwise None
    fn identify(buffer: &str) -> Token {
        // Change the buffer to lowercase to make it easier to compare
        let buffer_copied = buffer.to_owned().to_lowercase();

        match buffer_copied.as_str() {
            "let" => Token::new(TokenKind::Assign, buffer.to_string()),
            "func" => Token::new(TokenKind::Function, buffer.to_string()),
            "return" => Token::new(TokenKind::Return, buffer.to_string()),
            "import" => Token::new(TokenKind::Import, buffer.to_string()),
            _ => Token::new(TokenKind::Identifier, buffer.to_string()),
        }
    }
}
