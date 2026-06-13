use crate::{
    lexer::{Lexer, token::Token},
    parser::ast::{Expression, LetStatement, Program, Statement},
};

mod ast;
pub struct Parser<T: Iterator<Item = u8>> {
    lexer: Lexer<T>,
    next_token: Option<Token>,
}

impl<T: Iterator<Item = u8>> Parser<T> {
    pub fn new(mut lexer: Lexer<T>) -> Self {
        let next_token = lexer.next();
        Self { lexer, next_token }
    }
    fn read_next_token(&mut self) {
        self.next_token = self.lexer.next();
    }
    fn get_next_token(&self) -> Result<&Token, &'static str> {
        self.next_token
            .as_ref()
            .ok_or("No tokens to parse a statement")
    }

    fn parse_program(&mut self) -> Result<Program, &'static str> {
        let mut prog = vec![];
        while self.next_token.is_some() {
            prog.push(self.parse_statement()?);
            self.read_next_token();
        }
        Ok(prog)
    }

    fn parse_statement(&mut self) -> Result<Statement, &'static str> {
        match self.get_next_token()? {
            Token::Let => {
                self.read_next_token();
                Ok(Statement::Let(self.parse_let_statement()?))
            }
            _ => Err("Parsing Error: not a Statement"),
        }
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement, &'static str> {
        let ident = match self.get_next_token()? {
            Token::Ident(var) => Ok(var.clone()),
            _ => Err("Parsing Error: identifier missing in Let Statement"),
        }?;
        self.read_next_token();
        if !matches!(self.next_token, Some(Token::Assign)) {
            return Err("Parsing Error: no assignment found in Let Statement");
        }
        self.read_next_token();
        let val = self.parse_expression()?;
        self.read_next_token();
        if !matches!(self.next_token, Some(Token::Semicolon)) {
            return Err("Parsing Error: no semicolon found in Let Statement");
        }
        Ok(LetStatement {
            name: ident,
            value: val,
        })
    }

    fn parse_expression(&mut self) -> Result<Expression, &'static str> {
        match self.get_next_token()? {
            Token::Int(num) => Ok(Expression::Int(*num)),
            _ => Err("Parsing Error: Invalid Expression"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        lexer::Lexer,
        parser::{
            Parser,
            ast::{Expression, LetStatement, Statement},
        },
    };

    #[test]
    fn test_let_statements() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;";
        let mut p = Parser::new(Lexer::new(input.bytes()));
        let program = p.parse_program();
        assert!(
            program.is_ok(),
            "Received error: {}",
            program.err().unwrap()
        );
        let program = program.unwrap();
        assert_eq!(
            program.len(),
            3,
            "program doesn't contain 3 statements. got: {}",
            program.len()
        );
        let mut program = program.iter();
        assert_eq!(
            program.next(),
            Some(&Statement::Let(LetStatement {
                name: "x".to_string(),
                value: Expression::Int(5)
            }))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Let(LetStatement {
                name: "y".to_string(),
                value: Expression::Int(10)
            }))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Let(LetStatement {
                name: "foobar".to_string(),
                value: Expression::Int(838383)
            }))
        );
    }
}
