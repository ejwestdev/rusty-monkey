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

    fn read_identifier(&mut self) -> String {
        let start_position = self.position;
        while (self.ch as char).is_ascii_alphabetic() {
            self.read_char();
        }
        self.input[start_position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.ch as char, ' ' | '\t' | '\n' | '\r') {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> String {
        let start_position = self.position;
        while (self.ch as char).is_ascii_digit() {
            self.read_char();
        }
        self.input[start_position..self.position].to_string()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let letter = self.ch as char;
        let tok = match letter {
            '=' => Token {
                token_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            '+' => Token {
                token_type: TokenType::Plus,
                literal: "+".to_string(),
            },
            '-' => Token {
                token_type: TokenType::Minus,
                literal: "-".to_string(),
            },
            '!' => Token {
                token_type: TokenType::Bang,
                literal: "!".to_string(),
            },
            '/' => Token {
                token_type: TokenType::Slash,
                literal: "/".to_string(),
            },
            '*' => Token {
                token_type: TokenType::Asterisk,
                literal: "*".to_string(),
            },
            '<' => Token {
                token_type: TokenType::Lt,
                literal: "<".to_string(),
            },
            '>' => Token {
                token_type: TokenType::Gt,
                literal: ">".to_string(),
            },
            ';' => Token {
                token_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            ',' => Token {
                token_type: TokenType::Comma,
                literal: ",".to_string(),
            },
            '(' => Token {
                token_type: TokenType::Lparen,
                literal: "(".to_string(),
            },
            ')' => Token {
                token_type: TokenType::Rparen,
                literal: ")".to_string(),
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
            _ => {
                if letter.is_ascii_alphabetic() || letter == '_' {
                    let ident = self.read_identifier();
                    return Token {
                        token_type: Token::lookup_ident(&ident),
                        literal: ident,
                    };
                } else if letter.is_ascii_digit() {
                    let num = self.read_number();
                    return Token {
                        token_type: TokenType::Int(num.clone()),
                        literal: num,
                    };
                } else {
                    Token {
                        token_type: TokenType::Illegal,
                        literal: letter.to_string(),
                    }
                }
            }
        };
        self.read_char();
        tok
    }
}
