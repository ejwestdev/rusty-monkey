use crate::lexing::lexer_impl::Lexer;
use crate::parsing::parser::Parser;
use crate::token::TokenType;
use std::io::BufRead;
use std::io::Write;

const PROMPT: &str = ">> ";

const MONKEY_FACE: &str = r#"          __,__
  .--.  .-"     "-.  .--.
 / .. \/  .-. .-.  \/ .. \
| |  ' /   Y   Y   \  ' | |
 \  \  \ 0 | 0 /  /  /  /
  \ '-.\.-"""""""-./.-' /
   '-._'-.       .-'_.-'
"#;

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
        let mut parser = Parser::new(lexer.clone());
        if let Some(program) = Parser::parse_program(&mut parser) {
            if parser.errors.is_empty() {
                writer
                    .write_all(format!("{program:?}\n").as_bytes())
                    .expect("failed to write program");
            } else {
                for err in &parser.errors {
                    writer.write_all(MONKEY_FACE.as_bytes()).expect("monkey");
                    writer
                        .write_all(format!("\t{err}\n").as_bytes())
                        .expect("failed to write error");
                }
            }
        }
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
