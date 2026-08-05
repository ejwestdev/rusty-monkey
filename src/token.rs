#[derive(Debug, PartialEq)]
struct TokenError;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Illegal,
    Eof,

    //Identifiers + literals
    Ident,
    Int,

    //Operators
    Assign,
    Plus,

    //Delimiters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    //Keywords
    Function,
    Let,
}

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    fn check_identifier(value: &str) -> TokenType {
        match value {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            _ => TokenType::Ident,
        }
    }
}
