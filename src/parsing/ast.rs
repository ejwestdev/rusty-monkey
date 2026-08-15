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
    Prefix {
        operator: String,
        right: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
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
            Expression::Prefix { operator, right } => {
                format!("({operator}{})", right.string())
            }
            Expression::Infix {
                left,
                operator,
                right,
            } => format!(
                "({} {operator} {})",
                left.string(),
                right.string()
            ),
        }
    }
}

impl Program {
    pub fn string(&self) -> String {
        self.statements
            .iter()
            .map(Statement::string)
            .collect::<String>()
    }
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let { .. } => "let".to_string(),
            Statement::Return { .. } => "return".to_string(),
            Statement::Expression { expression, .. } => expression.token_literal().clone(),
        }
    }
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(s) => s.clone(),
            Expression::Integer(n) => n.to_string(),
            Expression::Prefix { right, .. } => right.token_literal(),
            Expression::Infix { left, .. } => left.token_literal(),
        }
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        self.statements
            .first()
            .map(Node::token_literal)
            .unwrap_or_default()
    }
}
