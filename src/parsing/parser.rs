use crate::lexing::lexer_impl;
use crate::parsing::ast::{self, Expression, Identifier, Program, Statement};
use crate::token;
use crate::token::TokenType;
use std::collections::HashMap;

type PrefixParseFn = fn(&mut Parser) -> Option<Expression>;
type InfixParseFn = fn(&mut Parser, Expression) -> Option<Expression>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
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
    pub fn new(lexer: lexer_impl::Lexer) -> Parser {
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
        parser.register_prefix(token::TokenType::True, Parser::parse_boolean);
        parser.register_prefix(token::TokenType::False, Parser::parse_boolean);
        parser.register_prefix(token::TokenType::Lparen, Parser::parse_grouped_expression);
        parser.register_prefix(token::TokenType::If, Parser::parse_if_expression);
        parser.register_prefix(token::TokenType::Bang, Parser::parse_prefix_expression);
        parser.register_prefix(token::TokenType::Minus, Parser::parse_prefix_expression);
        parser.register_infix(token::TokenType::Plus, Parser::parse_infix_expression);
        parser.register_infix(token::TokenType::Minus, Parser::parse_infix_expression);
        parser.register_infix(token::TokenType::Asterisk, Parser::parse_infix_expression);
        parser.register_infix(token::TokenType::Slash, Parser::parse_infix_expression);
        parser.register_infix(token::TokenType::Lt, Parser::parse_infix_expression);
        parser.register_infix(token::TokenType::Gt, Parser::parse_infix_expression);
        parser.register_infix(token::TokenType::Eq, Parser::parse_infix_expression);
        parser.register_infix(token::TokenType::NotEq, Parser::parse_infix_expression);
        parser
    }
    fn register_prefix(&mut self, t: TokenType, f: PrefixParseFn) {
        self.prefix_parse_fns.insert(t, f);
    }
    fn register_infix(&mut self, t: TokenType, f: InfixParseFn) {
        self.infix_parse_fns.insert(t, f);
    }
    fn next_token(&mut self) {
        self.cur_tok = self.peek_tok.clone();
        self.peek_tok = self.lexer.next_token();
    }
    fn cur_token_is(&self, token_type: TokenType) -> bool {
        self.cur_tok.token_type == token_type
    }
    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token().token_type == token_type
    }
    fn peek_token(&self) -> token::Token {
        self.peek_tok.clone()
    }
    fn peek_error(&mut self, t: TokenType) {
        let msg = format!(
            "Expected next token to be {:?}, got {:?} instead",
            t, self.peek_tok.token_type
        );
        self.errors.push(msg);
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
            token::TokenType::Return => Some(self.parse_return_statement()),
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
        let mut left = prefix(self)?;
        while precedence < self.peek_precedence() {
            let infix = *self.infix_parse_fns.get(&self.peek_tok.token_type)?;
            self.next_token();
            left = infix(self, left)?;
        }
        Some(left)
    }
    fn parse_identifier(parser: &mut Parser) -> Option<Expression> {
        Some(Expression::Identifier(parser.cur_tok.literal.clone()))
    }
    fn parse_integer_literal(parser: &mut Parser) -> Option<Expression> {
        Some(Expression::Integer(
            parser.cur_tok.literal.parse::<i64>().ok()?,
        ))
    }
    fn parse_boolean(parser: &mut Parser) -> Option<Expression> {
        Some(Expression::Boolean(
            parser.cur_tok.token_type == token::TokenType::True,
        ))
    }
    fn parse_grouped_expression(parser: &mut Parser) -> Option<Expression> {
        parser.next_token();
        let expr = parser.parse_expression(Precedence::Lowest)?;
        if !Self::expect_peek(parser, token::TokenType::Rparen) {
            return None;
        }
        Some(expr)
    }
    fn parse_if_expression(parser: &mut Parser) -> Option<Expression> {
        let token = parser.cur_tok.clone();
        if !Self::expect_peek(parser, token::TokenType::Lparen) {
            return None;
        }
        parser.next_token();
        let condition = parser.parse_expression(Precedence::Lowest)?;
        if !Self::expect_peek(parser, token::TokenType::Rparen) {
            return None;
        }
        if !Self::expect_peek(parser, token::TokenType::Lbrace) {
            return None;
        }
        let consequence = parser.parse_block_statement();
        let mut alternative = None;
        if parser.peek_token_is(token::TokenType::Else) {
            parser.next_token();
            if !Self::expect_peek(parser, token::TokenType::Lbrace) {
                return None;
            }
            alternative = Some(parser.parse_block_statement());
        }
        Some(Expression::If(Box::new(ast::IfExpression {
            token,
            condition,
            consequence,
            alternative,
        })))
    }
    fn parse_block_statement(&mut self) -> ast::BlockStatement {
        let token = self.cur_tok.clone();
        let mut statements = Vec::new();
        self.next_token();
        while !self.cur_token_is(token::TokenType::Rbrace) && !self.cur_token_is(token::TokenType::Eof) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }
        ast::BlockStatement { token, statements }
    }
    fn parse_prefix_expression(parser: &mut Parser) -> Option<Expression> {
        let operator = parser.cur_tok.literal.clone();
        parser.next_token();
        let right = parser.parse_expression(Precedence::Prefix)?;
        Some(Expression::Prefix {
            operator,
            right: Box::new(right),
        })
    }
    fn parse_infix_expression(parser: &mut Parser, left: Expression) -> Option<Expression> {
        let operator = parser.cur_tok.literal.clone();
        let precedence = parser.cur_precedence();
        parser.next_token();
        let right = parser.parse_expression(precedence)?;
        Some(Expression::Infix {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
    fn peek_precedence(&self) -> Precedence {
        self.precedence_for(self.peek_tok.token_type)
    }
    fn cur_precedence(&self) -> Precedence {
        self.precedence_for(self.cur_tok.token_type)
    }
    fn precedence_for(&self, token_type: TokenType) -> Precedence {
        match token_type {
            token::TokenType::Eq | token::TokenType::NotEq => Precedence::Equals,
            token::TokenType::Lt | token::TokenType::Gt => Precedence::LessGreater,
            token::TokenType::Plus | token::TokenType::Minus => Precedence::Sum,
            token::TokenType::Asterisk | token::TokenType::Slash => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }

    fn parse_return_statement(&mut self) -> Statement {
        while self.cur_tok.token_type != token::TokenType::Semicolon {
            self.next_token();
        }
        Statement::Return {
            token: token::TokenType::Return,
            return_value: Expression::Integer(0),
        }
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
