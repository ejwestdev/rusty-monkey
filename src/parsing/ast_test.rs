mod tests {
    use crate::parsing::ast::{Expression, Identifier, Program, Statement};
    use crate::token::{Token, TokenType};

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![Statement::Let {
                token: TokenType::Let,
                name: Identifier {
                    token: Token {
                        token_type: TokenType::Ident("myVar".to_string()),
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
