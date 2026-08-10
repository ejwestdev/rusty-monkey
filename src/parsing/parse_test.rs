#[cfg(test)]
mod tests {
    use crate::lexing::lexer_impl::Lexer;
    use crate::parsing::ast::Statement;
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
            None => panic!("parse_program returned None"),
        };

        assert_eq!(program.statements.len(), 3);
        let expected = ["x", "y", "foobar"];
        for (i, ident) in expected.iter().enumerate() {
            let Statement::Let { name, .. } = &program.statements[i] else {
                panic!("statement {} is not a Let", i);
            };
            assert_eq!(&name.value(), ident);
        }
    }
}
