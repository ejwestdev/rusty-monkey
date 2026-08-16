use crate::lexing::lexer_impl::Lexer;
use crate::parsing::parser::Parser;
use std::io::BufRead;
use std::io::Write;

const PROMPT: &str = ">> ";

const MONKEY_FACE: &str = r#"
            __,__
   .--.  .-"     "-.  .--.
  / .. \/  .-. .-.  \/ .. \
 | |  '|  /   Y   \  |'  | |
 | \   \  \ 0 | 0 /  /   / |
  \ '- ,\.-"`` ``"-./, -' /
   `'-' /_   ^ ^   _\ '-'`
       |  \._   _./  |
       \   \ `~` /   /
        '._ '-=-' _.'
           '~---~'`
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

        let lexer = Lexer::new(line);
        let mut parser = Parser::new(lexer);
        if let Some(program) = Parser::parse_program(&mut parser) {
            if parser.errors.is_empty() {
                writer
                    .write_all(program.string().as_bytes())
                    .expect("failed to write program");
                writer.write_all(b"\n").expect("failed to write newline");
            } else {
                print_parser_errors(writer, &parser.errors);
            }
        }
    }
}

fn print_parser_errors<W: Write>(writer: &mut W, errors: &[String]) {
    writer
        .write_all(MONKEY_FACE.as_bytes())
        .expect("failed to write monkey face");
    writer
        .write_all(b" parser errors:\n")
        .expect("failed to write error header");
    for err in errors {
        writer
            .write_all(format!("\t{err}\n").as_bytes())
            .expect("failed to write error");
    }
}

