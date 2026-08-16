use crate::token;

pub trait Node {
    fn token_literal(&self) -> String;
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
    pub fn string(&self) -> String {
        self.value.clone()
    }
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
            Statement::Expression { expression, .. } => format!("{};", expression.string()),
        }
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

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Integer(i64),
    Boolean(bool),
    Prefix {
        operator: String,
        right: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
    If(Box<IfExpression>),
    Function(Box<FunctionLiteral>),
}
impl Expression {
    pub fn string(&self) -> String {
        match self {
            Expression::Identifier(s) => s.clone(),
            Expression::Integer(n) => n.to_string(),
            Expression::Boolean(value) => value.to_string(),
            Expression::Prefix { operator, right } => {
                format!("({operator}{})", right.string())
            }
            Expression::Infix {
                left,
                operator,
                right,
            } => format!("({} {operator} {})", left.string(), right.string()),
            Expression::If(expr) => expr.string(),
            Expression::Function(expr) => expr.string(),
        }
    }
}
impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(s) => s.clone(),
            Expression::Integer(n) => n.to_string(),
            Expression::Boolean(value) => value.to_string(),
            Expression::Prefix { right, .. } => right.token_literal(),
            Expression::Infix { left, .. } => left.token_literal(),
            Expression::If(expr) => expr.token_literal(),
            Expression::Function(expr) => expr.token_literal(),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
impl Program {
    pub fn string(&self) -> String {
        self.statements
            .iter()
            .map(Statement::string)
            .collect::<String>()
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

#[derive(Debug)]
pub struct IfExpression {
    pub token: token::Token,
    pub condition: Expression,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}
impl IfExpression {
    pub fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    pub fn string(&self) -> String {
        let mut output = String::new();
        output.push_str("if");
        output.push_str(&self.condition.string());
        output.push(' ');
        output.push_str(&self.consequence.string());

        if let Some(alt) = &self.alternative {
            output.push_str("else ");
            output.push_str(&alt.string());
        }

        output
    }
}
#[derive(Debug)]
pub struct BlockStatement {
    pub token: token::Token,
    pub statements: Vec<Statement>,
}
impl BlockStatement {
    pub fn string(&self) -> String {
        self.statements
            .iter()
            .map(Statement::string)
            .collect::<String>()
    }
}
impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
#[derive(Debug)]
pub struct FunctionLiteral {
    pub token: token::Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}
impl FunctionLiteral {
    pub fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    pub fn string(&self) -> String {
        let params: Vec<String> = self.parameters.iter().map(Identifier::string).collect();
        format!(
            "{}({}) {}",
            self.token_literal(),
            params.join(", "),
            self.body.string()
        )
    }
}
