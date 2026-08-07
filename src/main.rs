use std::io;
use std::io::BufReader;
mod lexer;
mod parser;
mod repl;
fn main() {
    let mut reader = BufReader::new(io::stdin());
    let mut writer = io::stdout();
    repl::start(&mut reader, &mut writer);
}
