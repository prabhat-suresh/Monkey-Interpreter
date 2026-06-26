use std::io;

use crate::{
    evaluator::{eval, object::Object},
    lexer, parser,
};

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
        let output = match ast {
            Err(msg) => msg.to_string(),
            Ok(prog) => format!(
                "{}\n",
                match eval(prog) {
                    None => String::from(""),
                    Some(Object::Int(n)) => n.to_string(),
                    Some(Object::Bool(b)) => b.to_string(),
                    Some(Object::Null) => String::from("NULL"),
                }
            ),
        };
        writer
            .write_all(output.as_bytes())
            .expect("REPL failed to write output");
        writer.flush().expect("REPL failed to flush output");
    }
}
