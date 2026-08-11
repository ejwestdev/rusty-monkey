use crate::lexing::lexer_impl;
use crate::parsing::ast::{Expression, Identifier, Program, Statement};
use crate::token;
use crate::token::TokenType;

#[derive(Clone)]
pub struct Parser {
    pub lexer: lexer_impl::Lexer,

    pub cur_tok: token::Token,
    pub peek_tok: token::Token,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(mut lexer: lexer_impl::Lexer) -> Parser {
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
            errors: vec![],
        };
        parser.next_token();
        parser.next_token();
        parser
    }
    fn next_token(&mut self) {
        self.cur_tok = self.peek_tok.clone();
        self.peek_tok = self.lexer.next_token();
    }
    fn peek_token(&self) -> token::Token {
        self.peek_tok.clone()
    }
    fn peek_error(&mut self, t: TokenType) {
        let msg = format!(
            "Expected next token to be {:?}, got {:?} instead",
            t, self.peek_tok.token_type
        );
        self.errors.push(msg)
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
        let token::TokenType::Ident(name) = self.peek_token().token_type else {
            return None;
        };
        self.next_token();

        if self.peek_token().token_type != token::TokenType::Assign {
            return None;
        }
        self.next_token();

        while self.cur_tok.token_type != token::TokenType::Semicolon {
            self.next_token();
        }

        Some(Statement::Let {
            token: token::TokenType::Let,
            name: Identifier {
                token: token::Token {
                    token_type: token::TokenType::Ident(name.clone()),
                    literal: name.clone(),
                },
                value: name,
            },
            value: Expression::Integer(0),
        })
    }

    fn expect_peek(parser: &mut Parser, tok_type: TokenType) -> bool {
        if parser.peek_token().token_type == tok_type {
            parser.next_token();
            true
        } else {
            parser.peek_error(tok_type);
            false
        }
    }
}
