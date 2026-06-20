use std::io;

use crate::{lexer, parser};

const PROMPT: &str = ">> ";

pub fn start(mut reader: impl io::BufRead, mut writer: impl io::Write) {
    loop {
        writer
            .write_all(PROMPT.as_bytes())
            .expect("REPL failed to write output");
        writer.flush().expect("REPL failed to flush output");
        let mut buf = String::from("");
        reader
            .read_line(&mut buf)
            .expect("REPL failed to read line");

        let l = lexer::Lexer::new(buf.bytes());
        let mut p = parser::Parser::new(l);
        let ast = p.parse_program();
        writer
            .write_all(format!("{:?}\n", ast).as_bytes())
            .expect("REPL failed to write output");
        writer.flush().expect("REPL failed to flush output");
    }
}
