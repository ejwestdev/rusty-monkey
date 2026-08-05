#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Illegal,
    Eof,

    //Identifiers + literals
    Ident(String),
    Int(String),

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

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
}

impl Token {
    pub fn check_keyword_or_iden(value: &str) -> TokenType {
        match value {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            _ => TokenType::Ident(value.to_string()),
        }
    }
}
