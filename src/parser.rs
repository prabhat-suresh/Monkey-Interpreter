use crate::{
    lexer::{Lexer, token::Token},
    parser::ast::{Expression, LetStatement, PrefixExpression, Program, Statement},
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
        }
        Ok(prog)
    }

    fn parse_statement(&mut self) -> Result<Statement, &'static str> {
        match self.get_next_token()? {
            Token::Let => {
                self.read_next_token();
                let ls = self.parse_let_statement()?;
                self.read_next_token();
                Ok(Statement::Let(ls))
            }
            Token::Return => {
                self.read_next_token();
                let exp = self.parse_expression()?;
                self.read_next_token();
                if !matches!(self.next_token, Some(Token::Semicolon)) {
                    return Err("Parsing Error: no semicolon found in Let Statement");
                }
                self.read_next_token();
                Ok(Statement::Return(exp))
            }
            _ => {
                let exp = self.parse_expression()?;
                self.read_next_token();
                if matches!(self.next_token, Some(Token::Semicolon)) {
                    // As semicolons are optional in Expression Statements
                    self.read_next_token();
                }
                Ok(Statement::Expr(exp))
            }
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
        let next_token = self.get_next_token()?;
        match next_token {
            Token::Int(num) => Ok(Expression::Int(*num)),
            Token::Ident(var) => Ok(Expression::Ident(var.clone())),
            Token::Bang | Token::Minus => {
                let operator = next_token.clone();
                self.read_next_token();
                let exp = self.parse_expression()?;
                Ok(Expression::Prefix(PrefixExpression {
                    operator,
                    exp: Box::new(exp),
                }))
            }
            _ => Err("Parsing Error: Invalid Expression"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        lexer::{Lexer, token::Token},
        parser::{
            Parser,
            ast::{Expression, LetStatement, PrefixExpression, Statement},
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
        assert_eq!(program.next(), None);
    }

    #[test]
    fn test_return_statements() {
        let input = "
            return 5;
            return 10;
            return 993322;";
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
        assert_eq!(program.next(), Some(&Statement::Return(Expression::Int(5))));
        assert_eq!(
            program.next(),
            Some(&Statement::Return(Expression::Int(10)))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Return(Expression::Int(993322)))
        );
        assert_eq!(program.next(), None);
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
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
            1,
            "program doesn't contain 1 statement. got: {}",
            program.len()
        );
        let mut program = program.iter();
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Ident("foobar".to_string())))
        );
        assert_eq!(program.next(), None);
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";
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
            1,
            "program doesn't contain 1 statement. got: {}",
            program.len()
        );
        let mut program = program.iter();
        assert_eq!(program.next(), Some(&Statement::Expr(Expression::Int(5))));
        assert_eq!(program.next(), None);
    }

    #[test]
    fn test_prefix_expressions() {
        let input = "
            !5;
            -15;";
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
            2,
            "program doesn't contain 2 statements. got: {}",
            program.len()
        );
        let mut program = program.iter();
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Prefix(PrefixExpression {
                operator: Token::Bang,
                exp: Box::new(Expression::Int(5))
            })))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Prefix(PrefixExpression {
                operator: Token::Minus,
                exp: Box::new(Expression::Int(15))
            })))
        );
        assert_eq!(program.next(), None);
    }
}
