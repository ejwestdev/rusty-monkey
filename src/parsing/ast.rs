use crate::token;

pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String,
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
    Expression {
        token: token::Token,
        expression: Expression,
    },
}

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Integer(i64),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Identifier {
    pub fn value(&self) -> &String {
        &self.value
    }
    pub fn string(&self) -> String {
        self.value.clone()
    }
}

impl Statement {
    pub fn string(&self) -> String {
        match self {
            Statement::Let { name, value, .. } => format!(
                "{} {} = {};",
                self.token_literal(),
                name.string(),
                value.string()
            ),
            Statement::Return { .. } => "return".to_string(),
            Statement::Expression { .. } => "expression".to_string(),
        }
    }
}

impl Expression {
    pub fn string(&self) -> String {
        match self {
            Expression::Identifier(s) => s.clone(),
            Expression::Integer(n) => n.to_string(),
        }
    }
}

impl Program {
    pub fn string(&self) -> String {
        self.statements
            .iter()
            .map(|s| s.string())
            .collect::<Vec<String>>()
            .join("")
    }
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

impl Node for Program {
    fn token_literal(&self) -> String {
        self.statements
            .first()
            .map(|s| s.token_literal())
            .unwrap_or_default()
    }
}
