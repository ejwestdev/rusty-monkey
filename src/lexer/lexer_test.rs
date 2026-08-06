#[cfg(test)]
mod tests {
    use crate::lexer::lexer_impl::Lexer;
    use crate::lexer::token::TokenType;
    #[test]
    fn test_next_token() {
        let input = "let five = 5;
let ten = 10;
let add = fn(x,  y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;
";
        let cases = vec![
            (TokenType::Let, "let".to_string()),
            (TokenType::Ident("five".to_string()), "five".to_string()),
            (TokenType::Assign, "=".to_string()),
            (TokenType::Int("5".to_string()), "5".to_string()),
            (TokenType::Semicolon, ";".to_string()),
            (TokenType::Let, "let".to_string()),
            (TokenType::Ident("ten".to_string()), "ten".to_string()),
            (TokenType::Assign, "=".to_string()),
            (TokenType::Int("10".to_string()), "10".to_string()),
            (TokenType::Semicolon, ";".to_string()),
            (TokenType::Let, "let".to_string()),
            (TokenType::Ident("add".to_string()), "add".to_string()),
            (TokenType::Assign, "=".to_string()),
            (TokenType::Function, "fn".to_string()),
            (TokenType::Lparen, "(".to_string()),
            (TokenType::Ident("x".to_string()), "x".to_string()),
            (TokenType::Comma, ",".to_string()),
            (TokenType::Ident("y".to_string()), "y".to_string()),
            (TokenType::Rparen, ")".to_string()),
            (TokenType::Lbrace, "{".to_string()),
            (TokenType::Ident("x".to_string()), "x".to_string()),
            (TokenType::Plus, "+".to_string()),
            (TokenType::Ident("y".to_string()), "y".to_string()),
            (TokenType::Semicolon, ";".to_string()),
            (TokenType::Rbrace, "}".to_string()),
            (TokenType::Semicolon, ";".to_string()),
            (TokenType::Let, "let".to_string()),
            (TokenType::Ident("result".to_string()), "result".to_string()),
            (TokenType::Assign, "=".to_string()),
            (TokenType::Ident("add".to_string()), "add".to_string()),
            (TokenType::Lparen, "(".to_string()),
            (TokenType::Ident("five".to_string()), "five".to_string()),
            (TokenType::Comma, ",".to_string()),
            (TokenType::Ident("ten".to_string()), "ten".to_string()),
            (TokenType::Rparen, ")".to_string()),
            (TokenType::Semicolon, ";".to_string()),
            (TokenType::Bang, "!".to_string()),
            (TokenType::Minus, "-".to_string()),
            (TokenType::Slash, "/".to_string()),
            (TokenType::Asterisk, "*".to_string()),
            (TokenType::Int("5".to_string()), "5".to_string()),
            (TokenType::Semicolon, ";".to_string()),
            (TokenType::Int("5".to_string()), "5".to_string()),
            (TokenType::Lt, "<".to_string()),
            (TokenType::Int("10".to_string()), "10".to_string()),
            (TokenType::Gt, ">".to_string()),
            (TokenType::Int("5".to_string()), "5".to_string()),
            (TokenType::Semicolon, ";".to_string()),
            (TokenType::Eof, "".to_string()),
        ];
        let mut lexer = Lexer::new(input.to_string());
        for (expected_type, expected_literal) in cases {
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, expected_type);
            assert_eq!(tok.literal, expected_literal);
        }
    }
}
