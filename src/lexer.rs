mod token;
use self::token::Token;

struct Lexer<T: Iterator<Item = u8>> {
    src_code_iter: T,
    next_byte: Option<u8>,
}

fn is_letter(b: u8) -> bool {
    b == b'_' || b.is_ascii_alphabetic()
}

impl<T: Iterator<Item = u8>> Lexer<T> {
    pub fn new(mut src_code_iter: T) -> Self {
        let next_byte = src_code_iter.next();
        Self {
            src_code_iter,
            next_byte,
        }
    }
    fn read_identifier_or_keyword(&mut self) -> Token {
        let mut identifier = String::from("");
        while self.next_byte.is_some_and(is_letter) {
            identifier.push(self.next_byte.unwrap() as char);
            self.next_byte = self.src_code_iter.next();
        }
        match identifier.as_str() {
            "let" => Token::Let,
            "fn" => Token::Function,
            _ => Token::Ident(identifier),
        }
    }
    fn read_number(&mut self) -> i32 {
        let mut identifier = String::from("");
        while self.next_byte.is_some_and(|b| b.is_ascii_digit()) {
            identifier.push(self.next_byte.unwrap() as char);
            self.next_byte = self.src_code_iter.next();
        }
        identifier.parse().expect("Lexer couldn't parse integer")
    }
}

// As the book implements a NextToken method on Lexer, it would give us more functionality if we
// implement the Iterator trait itself which requires us to define a similar method anyway
impl<T: Iterator<Item = u8>> Iterator for Lexer<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut dont_iter = false;
        let iter_next = match self.next_byte? {
            b'=' => Token::Assign,
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b',' => Token::Comma,
            b'+' => Token::Plus,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b if is_letter(b) => {
                dont_iter = true;
                self.read_identifier_or_keyword()
            }
            b if b.is_ascii_digit() => {
                dont_iter = true;
                Token::Int(self.read_number())
            }
            b if b.is_ascii_whitespace() => {
                dont_iter = true;
                self.next_byte = self.src_code_iter.next();
                self.next()?
            }
            _ => Token::Illegal,
        };
        if !dont_iter {
            // iterate to next byte if it's not already done
            self.next_byte = self.src_code_iter.next();
        }
        Some(iter_next)
    }
}

#[cfg(test)]
mod test {
    use super::token::Token;

    use super::Lexer;

    #[test]
    fn next_token() {
        let input = "
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);";
        let mut l = Lexer::new(input.bytes());
        assert_eq!(l.next(), Some(Token::Let));
        assert_eq!(l.next(), Some(Token::Ident("five".to_string())));
        assert_eq!(l.next(), Some(Token::Assign));
        assert_eq!(l.next(), Some(Token::Int(5)));
        assert_eq!(l.next(), Some(Token::Semicolon));
        assert_eq!(l.next(), Some(Token::Let));
        assert_eq!(l.next(), Some(Token::Ident("ten".to_string())));
        assert_eq!(l.next(), Some(Token::Assign));
        assert_eq!(l.next(), Some(Token::Int(10)));
        assert_eq!(l.next(), Some(Token::Semicolon));
        assert_eq!(l.next(), Some(Token::Let));
        assert_eq!(l.next(), Some(Token::Ident("add".to_string())));
        assert_eq!(l.next(), Some(Token::Assign));
        assert_eq!(l.next(), Some(Token::Function));
        assert_eq!(l.next(), Some(Token::LParen));
        assert_eq!(l.next(), Some(Token::Ident("x".to_string())));
        assert_eq!(l.next(), Some(Token::Comma));
        assert_eq!(l.next(), Some(Token::Ident("y".to_string())));
        assert_eq!(l.next(), Some(Token::RParen));
        assert_eq!(l.next(), Some(Token::LBrace));
        assert_eq!(l.next(), Some(Token::Ident("x".to_string())));
        assert_eq!(l.next(), Some(Token::Plus));
        assert_eq!(l.next(), Some(Token::Ident("y".to_string())));
        assert_eq!(l.next(), Some(Token::Semicolon));
        assert_eq!(l.next(), Some(Token::RBrace));
        assert_eq!(l.next(), Some(Token::Semicolon));
        assert_eq!(l.next(), Some(Token::Let));
        assert_eq!(l.next(), Some(Token::Ident("result".to_string())));
        assert_eq!(l.next(), Some(Token::Assign));
        assert_eq!(l.next(), Some(Token::Ident("add".to_string())));
        assert_eq!(l.next(), Some(Token::LParen));
        assert_eq!(l.next(), Some(Token::Ident("five".to_string())));
        assert_eq!(l.next(), Some(Token::Comma));
        assert_eq!(l.next(), Some(Token::Ident("ten".to_string())));
        assert_eq!(l.next(), Some(Token::RParen));
        assert_eq!(l.next(), Some(Token::Semicolon));
        // We treat None like the EOF token used in the book
        assert_eq!(l.next(), None);
    }
}
