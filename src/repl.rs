use crate::lexer::lexer_impl::Lexer;
use crate::token::TokenType;
use std::io::BufRead;
use std::io::Write;

const PROMPT: &str = ">> ";

pub fn start<R: BufRead, W: Write>(reader: &mut R, writer: &mut W) {
    loop {
        writer
            .write_all(PROMPT.as_bytes())
            .and_then(|()| writer.flush())
            .expect("failed to write prompt");

        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) | Err(_) => return,
            Ok(_) => {}
        }

        let mut lexer = Lexer::new(line);
        loop {
            let tok = lexer.next_token();
            if tok.token_type == TokenType::Eof {
                break;
            }
            writer
                .write_all(format!("{tok:?}\n").as_bytes())
                .expect("failed to write token");
        }
    }
}
