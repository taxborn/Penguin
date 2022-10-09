use std::fmt;
use std::fs;
use std::path::PathBuf;

/// A position type to keep track of where we are in the source code.
type Position = (usize, usize);

#[derive(Debug)]
/// Errors that can occur during lexing.
pub enum LexerError<'error> {
    /// An invalid character was encountered.
    InvalidCharacter(&'error Location, char),
    /// An invalid identifier was encountered.
    InvalidIdentifier(&'error Location, String),
    /// An invalid escape sequence was encountered.
    InvalidEscapeSequence(&'error Location, char),
    /// Unexpected end of input.
    UnexpectedEOF(&'error Location),
}

impl<'error> fmt::Display for LexerError<'error> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerError::InvalidCharacter(loc, c) => {
                write!(
                    f,
                    "[{}:{}:{}] Invalid character '{}'.",
                    loc.source, loc.line, loc.column, c
                )
            }
            LexerError::InvalidIdentifier(loc, s) => {
                write!(
                    f,
                    "[{}:{}:{}] Invalid identifier '{}'.",
                    loc.source, loc.line, loc.column, s
                )
            }
            LexerError::InvalidEscapeSequence(loc, c) => {
                write!(
                    f,
                    "[{}:{}:{}] Invalid escape sequence '{}'.",
                    loc.source, loc.line, loc.column, c
                )
            }
            LexerError::UnexpectedEOF(loc) => {
                write!(
                    f,
                    "[{}:{}:{}] Unexpected end of file.",
                    loc.source, loc.line, loc.column
                )
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

    /// Function Function, // func
    Function, // func

    /// Return
    Return, // return

    /// Import
    Import, // import
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub index: usize,
    pub prev_line_length: usize,
    pub current_line_length: usize,
    pub source: String,
}

impl Location {
    pub fn new(line: usize, column: usize, source: String) -> Self {
        Self {
            line,
            column,
            index: 0,
            // TODO: We might want to rememeber all of the previous lines
            // lengths for diagnostics, but for now we only need the length of
            // the previous line
            prev_line_length: 0,
            current_line_length: 0,
            source,
        }
    }

    pub fn advance(&mut self, current: Option<char>) {
        if let Some(current) = current {
            if current == '\n' {
                self.line += 1;
                self.column = 0;
                self.prev_line_length = self.current_line_length;
                self.current_line_length = 0;
            } else {
                self.column += 1;
                self.current_line_length += 1;
            }

            self.index += 1;
        }
    }

    pub fn retreat(&mut self, current: Option<char>) {
        if let Some(current) = current {
            if current == '\n' {
                self.line -= 1;
                self.column = 0;
                self.current_line_length = self.prev_line_length;
                self.prev_line_length = 0;
            } else {
                self.column -= 1;
            }

            self.index -= 1;
        }
    }

    pub fn current_location(&self) -> Position {
        (self.line, self.column)
    }
}

#[derive(Debug)]
pub struct Lexer {
    pub loc: Location,
    source: Vec<char>,
    current: Option<char>,
}

impl Lexer {
    /// Create a new lexer from a string.
    pub fn new(file: PathBuf) -> Self {
        let source = fs::read_to_string(&file).unwrap().chars().collect();
        let file_name = file.file_name().unwrap().to_str().unwrap().to_string();

        Self {
            source,
            current: None,
            loc: Location::new(1, 0, file_name),
        }
    }

    pub fn lex_from_string(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            current: None,
            loc: Location::new(1, 0, "string".to_string()),
        }
    }

    /// Lex the source code into a list of tokens.
    pub fn lex(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = vec![];

        // While we are not at the end of the contents
        while self.source.len() > self.loc.index {
            let current = if let Some(current) = self.current_char() {
                self.current = Some(current);
                current
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
                    // Right now, the only way to tell if an assignment is
                    // typed or not is to check if the previous token is a
                    // TypeAssignment. If it is, then this is a TypedAssignment.
                    //
                    // I want to handle this logic in the ':' case, but I'm not
                    // sure how to do that yet, or if it's even possible.
                    // Maybe this is something that can be handled in the
                    // parser?
                    let previous_token = tokens.last().unwrap();

                    if previous_token.kind == TokenKind::TypeAssignment {
                        tokens.pop();

                        tokens.push(Token::new(TokenKind::UnTypedAssignment, ":=".to_string()));
                    } else {
                        tokens.push(Token::new(TokenKind::LetAssignment, current.to_string()));
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
                                    _ => {
                                        self.next();

                                        return Err(LexerError::InvalidEscapeSequence(
                                            &self.loc, next,
                                        ));
                                    }
                                }
                            }
                        } else {
                            buffer.push(next);
                        }

                        self.next();
                    }

                    // If we didn't find the end of the string, return an error
                    if !found_close {
                        return Err(LexerError::UnexpectedEOF(&self.loc));
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
                    self.next();

                    // Check if the next character is an equals sign, if so,
                    // this is a short increment
                    if let Some(next) = self.current_char() {
                        if next == '=' {
                            tokens.push(Token::new(TokenKind::ShortIncrement, "+=".to_string()));
                        } else {
                            // Otherwise, this is a normal plus. Also decrement
                            // the location so that the next token is not
                            // skipped
                            self.prev();

                            tokens.push(Token::new(TokenKind::Plus, current.to_string()));
                        }
                    }

                    self.next();
                }
                '-' => {
                    self.next();

                    // Check if the next character is an equals sign, if so,
                    // this is a short decrement
                    if let Some(next) = self.current_char() {
                        if next == '=' {
                            tokens.push(Token::new(TokenKind::ShortDecrement, "-=".to_string()));
                        } else {
                            // Otherwise, this is a normal minus. Also decrement
                            // the location so that the next token is not
                            // skipped
                            self.prev();

                            tokens.push(Token::new(TokenKind::Minus, current.to_string()));
                        }
                    }

                    self.next();
                }
                '*' => {
                    self.next();

                    // Check if the next character is an equals sign, if so,
                    // this is a short multiply
                    if let Some(next) = self.current_char() {
                        if next == '=' {
                            tokens.push(Token::new(TokenKind::ShortMultiply, "*=".to_string()));
                        } else {
                            // Otherwise, this is a normal multiplication. Also
                            // decrementthe location so that the next token is
                            // not skipped
                            self.prev();

                            tokens.push(Token::new(TokenKind::Multiply, current.to_string()));
                        }
                    }

                    self.next();
                }
                '%' => {
                    self.next();

                    // Check if the next character is an modulo, if so,
                    // this is a short modulo
                    if let Some(next) = self.current_char() {
                        if next == '=' {
                            tokens.push(Token::new(TokenKind::ShortModulo, "%=".to_string()));
                        } else {
                            // Otherwise, this is a normal modulo. Also decrement
                            // the location so that the next token is not
                            // skipped
                            self.prev();

                            tokens.push(Token::new(TokenKind::Modulo, current.to_string()));
                        }
                    }

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
                                return Err(LexerError::UnexpectedEOF(&self.loc));
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
                    self.next();

                    return Err(LexerError::InvalidCharacter(&self.loc, current));
                }
            }
        }

        Ok(tokens)
    }

    /// Get the current character in the source
    fn current_char(&self) -> Option<char> {
        self.source.get(self.loc.index).cloned()
    }

    /// Move the lexer to the next character
    fn next(&mut self) {
        self.loc.advance(self.current_char());
    }

    /// Move the lexer to the previous character
    fn prev(&mut self) {
        self.loc.retreat(self.current_char());
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
