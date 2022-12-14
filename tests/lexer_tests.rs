use penguin::lexer::{Lexer, Token, TokenKind};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assignment() {
        let mut lexer = Lexer::lex_from_string(":=".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![Token::new(TokenKind::UnTypedAssignment, ":=".to_string())];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_spaced_assignment() {
        let mut lexer = Lexer::lex_from_string(": =".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![Token::new(TokenKind::UnTypedAssignment, ":=".to_string())];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_typed_assignment() {
        let mut lexer = Lexer::lex_from_string(": u32 =".to_string());
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
    fn test_underscore_identifier() {
        let mut lexer = Lexer::lex_from_string("_".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![Token::new(TokenKind::Identifier, "_".to_string())];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_multiple_underscores() {
        let mut lexer = Lexer::lex_from_string("__foo__bar__baz____".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![Token::new(
            TokenKind::Identifier,
            "__foo__bar__baz____".to_string(),
        )];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_variable_assignment_with_underscore() {
        let mut lexer =
            Lexer::lex_from_string("let __foo__bar__baz____ : u32 = 123456;".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![
            Token::new(TokenKind::Assign, "let".to_string()),
            Token::new(TokenKind::Identifier, "__foo__bar__baz____".to_string()),
            Token::new(TokenKind::TypeAssignment, ":".to_string()),
            Token::new(TokenKind::Identifier, "u32".to_string()),
            Token::new(TokenKind::LetAssignment, "=".to_string()),
            Token::new(TokenKind::Number(123456), "123456".to_string()),
            Token::new(TokenKind::Semicolon, ";".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_variable_assignment_to_string() {
        let mut lexer = Lexer::lex_from_string("let x : = \"hello world\";".to_string());
        let tokens = lexer.lex().unwrap();

        let expected_tokens = vec![
            Token::new(TokenKind::Assign, "let".to_string()),
            Token::new(TokenKind::Identifier, "x".to_string()),
            Token::new(TokenKind::UnTypedAssignment, ":=".to_string()),
            Token::new(TokenKind::String, "hello world".to_string()),
            Token::new(TokenKind::Semicolon, ";".to_string()),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_escape_sequences() {
        let mut lexer = Lexer::lex_from_string("'Don\\'t'".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![Token::new(TokenKind::String, "Don't".to_string())];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_string_with_escaped_quotes() {
        let mut lexer = Lexer::lex_from_string("\"\\\"hello\\\"\"".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![Token::new(TokenKind::String, "\"hello\"".to_string())];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_string_with_mixed_quotes() {
        let mut lexer = Lexer::lex_from_string("\"Hello, 'world!'\"".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![Token::new(TokenKind::String, "Hello, 'world!'".to_string())];

        assert_eq!(tokens, expected);

        let mut lexer_flipped = Lexer::lex_from_string("'Hello, \"world!\"'".to_string());
        let tokens_flipped = lexer_flipped.lex().unwrap();

        let expected_flipped = vec![Token::new(
            TokenKind::String,
            "Hello, \"world!\"".to_string(),
        )];

        assert_eq!(tokens_flipped, expected_flipped);
    }

    #[test]
    fn test_string_with_escaped_backslash() {
        let mut lexer = Lexer::lex_from_string("\"\\\\hello\\\\\"".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![Token::new(TokenKind::String, "\\hello\\".to_string())];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_number() {
        let mut lexer = Lexer::lex_from_string("123".to_string());
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
        let mut lexer = Lexer::lex_from_string("1_000".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![Token::new(TokenKind::Number(1000), "1_000".to_string())];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_short_increment() {
        let mut lexer = Lexer::lex_from_string("x += 5;".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![
            Token::new(TokenKind::Identifier, "x".to_string()),
            Token::new(TokenKind::ShortIncrement, "+=".to_string()),
            Token::new(TokenKind::Number(5), "5".to_string()),
            Token::new(TokenKind::Semicolon, ";".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_short_decrement() {
        let mut lexer = Lexer::lex_from_string("x -= 5;".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![
            Token::new(TokenKind::Identifier, "x".to_string()),
            Token::new(TokenKind::ShortDecrement, "-=".to_string()),
            Token::new(TokenKind::Number(5), "5".to_string()),
            Token::new(TokenKind::Semicolon, ";".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_case_insensitive_keywords() {
        let mut lexer = Lexer::lex_from_string("LET x : = 123;".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![
            Token::new(TokenKind::Assign, "LET".to_string()),
            Token::new(TokenKind::Identifier, "x".to_string()),
            Token::new(TokenKind::UnTypedAssignment, ":=".to_string()),
            Token::new(TokenKind::Number(123), "123".to_string()),
            Token::new(TokenKind::Semicolon, ";".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_that_we_can_have_numbers_and_letters() {
        let mut lexer = Lexer::lex_from_string("x123".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![Token::new(TokenKind::Identifier, "x123".to_string())];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_string_ending_with_backslash() {
        let mut lexer = Lexer::lex_from_string("\"hello \\".to_string());

        // We expect an error on lexing
        assert!(lexer.lex().is_err());
    }

    #[test]
    fn test_string_with_no_end_quote() {
        let mut lexer = Lexer::lex_from_string("\"hello".to_string());

        // We expect an error on lexing
        assert!(lexer.lex().is_err());
    }

    #[test]
    fn test_string_with_no_start_quote() {
        let mut lexer = Lexer::lex_from_string("hello\"".to_string());

        // We expect an error on lexing
        assert!(lexer.lex().is_err());
    }

    #[test]
    fn test_number_with_no_digits() {
        let mut lexer = Lexer::lex_from_string("1____".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![Token::new(TokenKind::Number(1), "1____".to_string())];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_ending_with_comment() {
        let mut lexer = Lexer::lex_from_string("x := 123; // This is a comment".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![
            Token::new(TokenKind::Identifier, "x".to_string()),
            Token::new(TokenKind::UnTypedAssignment, ":=".to_string()),
            Token::new(TokenKind::Number(123), "123".to_string()),
            Token::new(TokenKind::Semicolon, ";".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_inline_commenting() {
        let mut lexer =
            Lexer::lex_from_string("let __foo__bar__baz____ : /* u32 */ = 123;".to_string());

        let tokens = lexer.lex().unwrap();

        let expected = vec![
            Token::new(TokenKind::Assign, "let".to_string()),
            Token::new(TokenKind::Identifier, "__foo__bar__baz____".to_string()),
            Token::new(TokenKind::UnTypedAssignment, ":=".to_string()),
            Token::new(TokenKind::Number(123), "123".to_string()),
            Token::new(TokenKind::Semicolon, ";".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_inline_commenting_with_no_end() {
        let mut lexer =
            Lexer::lex_from_string("let __foo__bar__baz____ : /* u32 = 123;".to_string());

        // We expect an error on lexing
        assert!(lexer.lex().is_err());
    }

    #[test]
    fn test_arithmetic_lexing() {
        let mut lexer = Lexer::lex_from_string("1+2-3*4/5%6+=7-=8*=9/=1%=".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![
            Token::new(TokenKind::Number(1), "1".to_string()),
            Token::new(TokenKind::Plus, "+".to_string()),
            Token::new(TokenKind::Number(2), "2".to_string()),
            Token::new(TokenKind::Minus, "-".to_string()),
            Token::new(TokenKind::Number(3), "3".to_string()),
            Token::new(TokenKind::Multiply, "*".to_string()),
            Token::new(TokenKind::Number(4), "4".to_string()),
            Token::new(TokenKind::Divide, "/".to_string()),
            Token::new(TokenKind::Number(5), "5".to_string()),
            Token::new(TokenKind::Modulo, "%".to_string()),
            Token::new(TokenKind::Number(6), "6".to_string()),
            Token::new(TokenKind::ShortIncrement, "+=".to_string()),
            Token::new(TokenKind::Number(7), "7".to_string()),
            Token::new(TokenKind::ShortDecrement, "-=".to_string()),
            Token::new(TokenKind::Number(8), "8".to_string()),
            Token::new(TokenKind::ShortMultiply, "*=".to_string()),
            Token::new(TokenKind::Number(9), "9".to_string()),
            Token::new(TokenKind::ShortDivide, "/=".to_string()),
            Token::new(TokenKind::Number(1), "1".to_string()),
            Token::new(TokenKind::ShortModulo, "%=".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_readme_example() {
        let mut lexer = Lexer::lex_from_string("let x:u32=5;".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![
            Token::new(TokenKind::Assign, "let".to_string()),
            Token::new(TokenKind::Identifier, "x".to_string()),
            Token::new(TokenKind::TypeAssignment, ":".to_string()),
            Token::new(TokenKind::Identifier, "u32".to_string()),
            Token::new(TokenKind::LetAssignment, "=".to_string()),
            Token::new(TokenKind::Number(5), "5".to_string()),
            Token::new(TokenKind::Semicolon, ";".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_open_close_braces() {
        let mut lexer = Lexer::lex_from_string("(){}[]".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![
            Token::new(TokenKind::OpenParen, "(".to_string()),
            Token::new(TokenKind::CloseParen, ")".to_string()),
            Token::new(TokenKind::OpenBrace, "{".to_string()),
            Token::new(TokenKind::CloseBrace, "}".to_string()),
            Token::new(TokenKind::OpenBracket, "[".to_string()),
            Token::new(TokenKind::CloseBracket, "]".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_function_lexing() {
        let mut lexer = Lexer::lex_from_string("func main() := {}".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![
            Token::new(TokenKind::Function, "func".to_string()),
            Token::new(TokenKind::Identifier, "main".to_string()),
            Token::new(TokenKind::OpenParen, "(".to_string()),
            Token::new(TokenKind::CloseParen, ")".to_string()),
            Token::new(TokenKind::UnTypedAssignment, ":=".to_string()),
            Token::new(TokenKind::OpenBrace, "{".to_string()),
            Token::new(TokenKind::CloseBrace, "}".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_function_case_sensitivity() {
        let mut lexer = Lexer::lex_from_string("FuNc main() := {}".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![
            Token::new(TokenKind::Function, "FuNc".to_string()),
            Token::new(TokenKind::Identifier, "main".to_string()),
            Token::new(TokenKind::OpenParen, "(".to_string()),
            Token::new(TokenKind::CloseParen, ")".to_string()),
            Token::new(TokenKind::UnTypedAssignment, ":=".to_string()),
            Token::new(TokenKind::OpenBrace, "{".to_string()),
            Token::new(TokenKind::CloseBrace, "}".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_function_with_return_type() {
        let mut lexer = Lexer::lex_from_string("func main() : u32 = {}".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![
            Token::new(TokenKind::Function, "func".to_string()),
            Token::new(TokenKind::Identifier, "main".to_string()),
            Token::new(TokenKind::OpenParen, "(".to_string()),
            Token::new(TokenKind::CloseParen, ")".to_string()),
            Token::new(TokenKind::TypeAssignment, ":".to_string()),
            Token::new(TokenKind::Identifier, "u32".to_string()),
            Token::new(TokenKind::LetAssignment, "=".to_string()),
            Token::new(TokenKind::OpenBrace, "{".to_string()),
            Token::new(TokenKind::CloseBrace, "}".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_function_with_return_type_and_parameters() {
        let mut lexer = Lexer::lex_from_string("func main() : u32 = { return 5; }".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![
            Token::new(TokenKind::Function, "func".to_string()),
            Token::new(TokenKind::Identifier, "main".to_string()),
            Token::new(TokenKind::OpenParen, "(".to_string()),
            Token::new(TokenKind::CloseParen, ")".to_string()),
            Token::new(TokenKind::TypeAssignment, ":".to_string()),
            Token::new(TokenKind::Identifier, "u32".to_string()),
            Token::new(TokenKind::LetAssignment, "=".to_string()),
            Token::new(TokenKind::OpenBrace, "{".to_string()),
            Token::new(TokenKind::Return, "return".to_string()),
            Token::new(TokenKind::Number(5), "5".to_string()),
            Token::new(TokenKind::Semicolon, ";".to_string()),
            Token::new(TokenKind::CloseBrace, "}".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_importing() {
        let mut lexer = Lexer::lex_from_string("import \"test\";".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![
            Token::new(TokenKind::Import, "import".to_string()),
            Token::new(TokenKind::String, "test".to_string()),
            Token::new(TokenKind::Semicolon, ";".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_functions_with_multiple_parameters() {
        let mut lexer =
            Lexer::lex_from_string("func main(a: u32, b: u32) : u32 = { return 5; }".to_string());
        let tokens = lexer.lex().unwrap();

        let expected = vec![
            Token::new(TokenKind::Function, "func".to_string()),
            Token::new(TokenKind::Identifier, "main".to_string()),
            Token::new(TokenKind::OpenParen, "(".to_string()),
            Token::new(TokenKind::Identifier, "a".to_string()),
            Token::new(TokenKind::TypeAssignment, ":".to_string()),
            Token::new(TokenKind::Identifier, "u32".to_string()),
            Token::new(TokenKind::Comma, ",".to_string()),
            Token::new(TokenKind::Identifier, "b".to_string()),
            Token::new(TokenKind::TypeAssignment, ":".to_string()),
            Token::new(TokenKind::Identifier, "u32".to_string()),
            Token::new(TokenKind::CloseParen, ")".to_string()),
            Token::new(TokenKind::TypeAssignment, ":".to_string()),
            Token::new(TokenKind::Identifier, "u32".to_string()),
            Token::new(TokenKind::LetAssignment, "=".to_string()),
            Token::new(TokenKind::OpenBrace, "{".to_string()),
            Token::new(TokenKind::Return, "return".to_string()),
            Token::new(TokenKind::Number(5), "5".to_string()),
            Token::new(TokenKind::Semicolon, ";".to_string()),
            Token::new(TokenKind::CloseBrace, "}".to_string()),
        ];

        assert_eq!(tokens, expected);
    }
}
