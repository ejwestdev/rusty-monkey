use crate::lexing::lexer_impl;
use crate::parsing::ast::Program;
use crate::token;

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
    pub fn parse_program(parser: Parser) -> Option<Program> {
        todo!("parse_program")
    }
}
