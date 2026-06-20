#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    // Eof, not needed since we have implemented the iterator trait for the lexer which returns a None
    // when input is exhausted. We treat None as Eof and hence don't require another token for it

    // Identifiers and Literals
    Ident(String),
    Int(i32),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LT,
    GT,

    Eq,
    NEq,

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
    True,
    False,
    If,
    Else,
    Return,
}

impl Token {
    pub fn is_infix_operator(&self) -> bool {
        matches!(
            self,
            Token::Plus
                | Token::Minus
                | Token::Asterisk
                | Token::Slash
                | Token::GT
                | Token::LT
                | Token::Eq
                | Token::NEq
                | Token::LParen
        )
    }
}
