use crate::lexing::lexer_impl;
use crate::parsing::ast::{self, Expression, Identifier, Program, Statement};
use crate::token;
use crate::token::TokenType;
use std::collections::HashMap;

type PrefixParseFn = fn(&mut Parser) -> Option<Expression>;
type InfixParseFn = fn(&mut Parser, Expression) -> Option<Expression>;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest = 1,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
}

pub struct Parser {
    pub lexer: lexer_impl::Lexer,
    pub errors: Vec<String>,
    pub cur_tok: token::Token,
    pub peek_tok: token::Token,
    pub prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    pub infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

impl Parser {
    pub fn new(mut lexer: lexer_impl::Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            errors: vec![],
            cur_tok: token::Token {
                token_type: token::TokenType::Illegal,
                literal: String::new(),
            },
            peek_tok: token::Token {
                token_type: token::TokenType::Illegal,
                literal: String::new(),
            },
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
        parser.next_token();
        parser.next_token();
        parser.register_prefix(token::TokenType::Ident, Parser::parse_identifier);
        parser.register_prefix(token::TokenType::Int, Parser::parse_integer_literal);
        parser
    }
    fn register_prefix(&mut self, t: TokenType, f: PrefixParseFn) {
        self.prefix_parse_fns.insert(t, f);
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
            token::TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let stmt = ast::Statement::Expression {
            token: self.cur_tok.clone(),
            expression: self.parse_expression(Precedence::Lowest)?,
        };

        if self.peek_token().token_type == token::TokenType::Semicolon {
            self.next_token();
        }
        Some(stmt)
    }
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let prefix = *self.prefix_parse_fns.get(&self.cur_tok.token_type)?;
        prefix(self)
    }
    fn parse_identifier(parser: &mut Parser) -> Option<Expression> {
        Some(Expression::Identifier(parser.cur_tok.literal.clone()))
    }
    fn parse_integer_literal(parser: &mut Parser) -> Option<Expression> {
        Some(Expression::Integer(
            parser.cur_tok.literal.parse::<i64>().ok()?,
        ))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        while self.cur_tok.token_type != token::TokenType::Semicolon {
            self.next_token();
        }
        Some(Statement::Return {
            token: token::TokenType::Return,
            return_value: Expression::Integer(0),
        })
    }
    fn parse_let_statement(&mut self) -> Option<Statement> {
        let name = self.peek_token().literal.clone();

        if !Self::expect_peek(self, token::TokenType::Ident) {
            return None;
        }

        if !Self::expect_peek(self, token::TokenType::Assign) {
            return None;
        }

        while self.cur_tok.token_type != token::TokenType::Semicolon {
            self.next_token();
        }

        Some(Statement::Let {
            token: token::TokenType::Let,
            name: Identifier {
                token: token::Token {
                    token_type: token::TokenType::Ident,
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
