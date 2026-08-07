use crate::token;
pub trait Node {
    fn token_literal(&self) -> String;
}

pub enum Statement {
    Let {
        token: token::TokenType,
        //name: Identifier,
        value: Expression,
    },
    Return {
        token: token::TokenType,
        return_value: Expression,
    },
    Expression(Expression),
}

pub enum Expression {
    Identifier(String),
    Integer(i64),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        todo!("implement token_literal for Statement")
    }
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        todo!("implement token_literal for Expression")
    }
}
