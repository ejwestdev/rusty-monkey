use crate::token;
pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
    Let {
        token: token::TokenType,
        name: Identifier,
        value: Expression,
    },
    Return {
        token: token::TokenType,
        return_value: Expression,
    },
    Expression(Expression),
}

#[derive(Debug)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl Identifier {
    pub fn value(&self) -> &String {
        &self.value
    }
}
#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Integer(i64),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let { .. } => "let".to_string(),
            Statement::Return { .. } => "return".to_string(),
            Statement::Expression { .. } => "expression".to_string(),
        }
    }
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(..) => "identifier".to_string(),
            Expression::Integer(..) => "integer".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        self.statements
            .first()
            .map(|s| s.token_literal())
            .unwrap_or_default()
    }
}
