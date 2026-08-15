#[cfg(test)]
mod tests {
    use crate::lexing::lexer_impl::Lexer;
    use crate::parsing::ast::{Expression, Node, Statement};
    use crate::parsing::parser::Parser;

    #[test]
    fn test_let_statement() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        ";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = match Parser::parse_program(&mut parser) {
            Some(program) => program,
            None => panic!("parse_program returned None in test_let_statement"),
        };
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 3);
        let expected = ["x", "y", "foobar"];
        for (i, ident) in expected.iter().enumerate() {
            let Statement::Let { name, .. } = &program.statements[i] else {
                panic!("statement {} is not a Let", i);
            };
            assert_eq!(&name.value(), ident);
        }
    }
    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = match Parser::parse_program(&mut parser) {
            Some(program) => program,
            None => panic!("parse_program returned None in test_integer_literal_expression"),
        };
        assert_eq!(program.statements.len(), 1);
        for stmt in &program.statements {
            let Statement::Expression { .. } = stmt else {
                panic!("Statement not Statement::Expression, got {stmt:?} instead");
            };
            assert_eq!(stmt.token_literal(), "5");
        }
    }
    #[test]
    fn test_return_statement() {
        let input = "
        return 5;
        return 10;
        return 993322;
";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = match Parser::parse_program(&mut parser) {
            Some(program) => program,
            None => panic!("parse_program returned none in test_return_statement"),
        };
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 3);

        for stmt in &program.statements {
            let Statement::Return { .. } = stmt else {
                panic!("statement not Statement::Return. got={stmt:?}");
            };
            assert_eq!(stmt.token_literal(), "return");
        }
    }
    #[test]
    fn test_parsing_prefix_expressions() {
        let prefix_tests = vec![
            ("!5;", "!", 5i64),
            ("-15;", "-", 15i64),
        ];
        for (input, expected_operator, expected_value) in prefix_tests {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let program = Parser::parse_program(&mut parser)
                .expect("parse_program returned None in test_parsing_prefix_expressions");
            check_parser_errors(&parser);

            assert_eq!(program.statements.len(), 1);

            let Some(Statement::Expression { expression, .. }) = program.statements.first() else {
                panic!("program.statements is empty");
            };
            let Expression::Prefix { operator, right } = expression else {
                panic!("expression is not a Prefix. got={expression:?}");
            };
            assert_eq!(operator, expected_operator);

            let Expression::Integer(value) = **right else {
                panic!("right is not an Integer. got={right:?}");
            };
            assert_eq!(value, expected_value);
        }
    }
    #[test]
    fn test_identifier_expression() {
        let input = "foobar;".to_string();
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = match Parser::parse_program(&mut parser) {
            Some(program) => program,
            None => panic!("parse_program returned None in test_identifier_expression"),
        };
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        for stmt in &program.statements {
            let Statement::Expression { .. } = stmt else {
                panic!("statement not Statement::Expression. got={stmt:?}");
            };
            assert_eq!(stmt.token_literal(), "foobar");
        }
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let infix_tests = vec![
            ("5 + 5;", 5i64, "+", 5i64),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];
        for (input, expected_left, expected_operator, expected_right) in infix_tests {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let program = Parser::parse_program(&mut parser)
                .expect("parse_program returned None in test_parsing_infix_expressions");
            check_parser_errors(&parser);

            assert_eq!(program.statements.len(), 1);

            let Some(Statement::Expression { expression, .. }) = program.statements.first() else {
                panic!("program.statements is empty");
            };
            let Expression::Infix {
                left,
                operator,
                right,
            } = expression
            else {
                panic!("expression is not an Infix. got={expression:?}");
            };
            assert_eq!(operator, expected_operator);

            let Expression::Integer(left_value) = **left else {
                panic!("left is not an Integer. got={left:?}");
            };
            assert_eq!(left_value, expected_left);

            let Expression::Integer(right_value) = **right else {
                panic!("right is not an Integer. got={right:?}");
            };
            assert_eq!(right_value, expected_right);
        }
    }

    #[test]
    fn test_parsing_boolean_expressions() {
        for (input, expected) in [("true;", true), ("false;", false)] {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let program = Parser::parse_program(&mut parser)
                .expect("parse_program returned None in test_parsing_boolean_expressions");
            check_parser_errors(&parser);

            assert_eq!(program.statements.len(), 1);
            let Some(Statement::Expression { expression, .. }) = program.statements.first() else {
                panic!("program.statements is empty");
            };
            let Expression::Boolean(value) = expression else {
                panic!("expression is not a Boolean. got={expression:?}");
            };
            assert_eq!(*value, expected);
            assert_eq!(stmt_token_literal(expression), input.trim_end_matches(';'));
        }
    }

    fn stmt_token_literal(expression: &Expression) -> String {
        use crate::parsing::ast::Node;
        expression.token_literal()
    }

    fn check_parser_errors(parser: &Parser) {
        let errors = &parser.errors;
        if errors.is_empty() {
            return;
        }
        let messages: Vec<String> = errors
            .iter()
            .map(|msg| format!("parser error: {msg:?}"))
            .collect();
        panic!(
            "parser has {} errors:\n{}",
            errors.len(),
            messages.join("\n")
        );
    }
}
