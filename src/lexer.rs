mod token;
use self::token::Token;

struct Lexer<T: Iterator<Item = u8>> {
    src_code_iter: T,
}

impl<T: Iterator<Item = u8>> Lexer<T> {
    pub fn new(src_code_iter: T) -> Self {
        Self { src_code_iter }
    }
}

// As the book implements a NextToken method on Lexer, it would give us more functionality if we
// implement the Iterator trait itself which requires us to define a similar method anyway
impl<T: Iterator<Item = u8>> Iterator for Lexer<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.src_code_iter.next() {
            None => None,
            Some(b'=') => Some(Token::Assign),
            Some(b';') => Some(Token::Semicolon),
            Some(b'(') => Some(Token::LParen),
            Some(b')') => Some(Token::RParen),
            Some(b',') => Some(Token::Comma),
            Some(b'+') => Some(Token::Plus),
            Some(b'{') => Some(Token::LBrace),
            Some(b'}') => Some(Token::RBrace),
            _ => Some(Token::Illegal),
        }
    }
}

#[cfg(test)]
mod test {
    use super::token::Token;

    use super::Lexer;

    #[test]
    fn next_token() {
        let input = "=+(){},;";
        let mut l = Lexer::new(input.bytes());
        assert_eq!(l.next(), Some(Token::Assign));
        assert_eq!(l.next(), Some(Token::Plus));
        assert_eq!(l.next(), Some(Token::LParen));
        assert_eq!(l.next(), Some(Token::RParen));
        assert_eq!(l.next(), Some(Token::LBrace));
        assert_eq!(l.next(), Some(Token::RBrace));
        assert_eq!(l.next(), Some(Token::Comma));
        assert_eq!(l.next(), Some(Token::Semicolon));
        // We treat None like the EOF token used in the book
        assert_eq!(l.next(), None);
    }
}
