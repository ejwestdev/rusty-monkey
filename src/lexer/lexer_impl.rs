use crate::lexer::token::Token;
use crate::lexer::token::TokenType;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}
impl Lexer {
    pub fn new(new_input: String) -> Lexer {
        let mut l = Lexer {
            input: new_input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let tok = match self.ch as char {
            '=' => Token {
                token_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            ';' => Token {
                token_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            '(' => Token {
                token_type: TokenType::Lparen,
                literal: "(".to_string(),
            },
            ')' => Token {
                token_type: TokenType::Rparen,
                literal: ")".to_string(),
            },
            ',' => Token {
                token_type: TokenType::Comma,
                literal: ",".to_string(),
            },
            '+' => Token {
                token_type: TokenType::Plus,
                literal: "+".to_string(),
            },
            '{' => Token {
                token_type: TokenType::Lbrace,
                literal: "{".to_string(),
            },
            '}' => Token {
                token_type: TokenType::Rbrace,
                literal: "}".to_string(),
            },
            '\0' => Token {
                token_type: TokenType::Eof,
                literal: "".to_string(),
            },
            _ => Token {
                token_type: TokenType::Illegal,
                literal: (self.ch as char).to_string(),
            },
        };
        self.read_char();
        tok
    }
}
