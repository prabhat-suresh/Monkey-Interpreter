#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    // Eof, not needed since we have implemented the iterator trait for the lexer which returns a None
    // when input is exhausted. We treat None as Eof and hence don't require another token for it

    // Identifiers and Literals
    Ident(&'static str),
    Int(i32),

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
}
