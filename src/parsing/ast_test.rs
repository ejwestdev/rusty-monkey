mod tests {
    #[test]
    fn test_string() {
        use crate::parsing::ast::Expression;
        use crate::parsing::ast::Identifier;
        use crate::parsing::ast::Program;
        use crate::parsing::ast::Statement;
        use crate::token::Token;
        use crate::token::TokenType;

        let program = Program {
            statements: vec![Statement::Let {
                token: TokenType::Let,
                name: Identifier {
                    token: Token {
                        token_type: TokenType::Ident,
                        literal: "myVar".to_string(),
                    },
                    value: "myVar".to_string(),
                },
                value: Expression::Identifier("anotherVar".to_string()),
            }],
        };

        assert_eq!(program.string(), "let myVar = anotherVar;");
    }
}
