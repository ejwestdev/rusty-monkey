#[cfg(test)]
mod tests {
    use crate::token::TokenType;
    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let cases = vec![
            (TokenType::Assign, "=".to_string()),
            (TokenType::Plus, "+".to_string()),
            (TokenType::Lparen, "(".to_string()),
            (TokenType::Rparen, ")".to_string()),
            (TokenType::Rbrace, "{".to_string()),
            (TokenType::Lbrace, "}".to_string()),
            (TokenType::Comma, ",".to_string()),
            (TokenType::Semicolon, ";".to_string()),
            (TokenType::Eof, "".to_string()),
        ];
        let lexer = New(input);
        for (expected_type, expected_literal) in cases {
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, expected_type);
            assert_eq!(tok.literal, expected_literal);
        }
    }
}
