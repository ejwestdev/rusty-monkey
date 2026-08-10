use crate::lexing::lexer_impl;
use crate::parsing::ast::Program;
use crate::parsing::ast::Statement;
use crate::token;
use crate::token::Token;

pub struct Parser {
    pub lexer: lexer_impl::Lexer,

    pub cur_tok: token::Token,
    pub peek_tok: token::Token,
}

impl Parser {
    pub fn new(mut lexer: lexer_impl::Lexer) -> Parser {
        let next_tok = lexer.next_token();
        let mut parser = Parser {
            lexer,
            cur_tok: token::Token {
                token_type: token::TokenType::Illegal,
                literal: String::new(),
            },
            peek_tok: token::Token {
                token_type: token::TokenType::Illegal,
                literal: String::new(),
            },
        };
        parser.next_token();
        parser.next_token();
        parser
    }
    fn next_token(&mut self) {
        self.cur_tok = self.peek_tok.clone();
        self.peek_tok = self.lexer.next_token();
    }
    pub fn parse_program(parser: &mut Parser) -> Option<Program> {
        let mut program: Program = Program { statements: vec![] };
        while parser.cur_tok.token_type != token::TokenType::Eof {
            if let Some(statement) = parser.parse_statement() {
                program.statements.push(statement);
            }
            parser.next_token();
        }
        Some(program)
    }
    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_tok.token_type {
            token::TokenType::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let statement = self.parse_statement().unwrap();
        match self.tok
        todo!()
    }

    fn expect_peek(self, tt: token::TokenType) -> bool {
        todo!()
    }
}
