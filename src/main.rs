use std::io::{self, BufReader, BufWriter};

mod lexer;
mod repl;
fn main() {
    println!(
        "Hello {}! This is the Monkey programming language!",
        env!("USER")
    );
    println!("Feel free to type in commands");
    repl::start(BufReader::new(io::stdin()), BufWriter::new(io::stdout()));
}
