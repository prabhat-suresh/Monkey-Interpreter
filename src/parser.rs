use crate::{
    lexer::{Lexer, token::Token},
    parser::ast::{
        Expression, InfixExpression, LetStatement, PrefixExpression, Program, Statement,
    },
};

mod ast;
pub struct Parser<T: Iterator<Item = u8>> {
    lexer: Lexer<T>,
    next_token: Option<Token>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Precedence {
    fn precedence_of_infix_operator(tok: &Token) -> Precedence {
        match tok {
            Token::Eq | Token::NEq => Self::Equals,
            Token::LT | Token::GT => Self::LessGreater,
            Token::Plus | Token::Minus => Self::Sum,
            Token::Asterisk | Token::Slash => Self::Product,
            _ => panic!("Not an infix operator"),
        }
    }
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
                let exp = self.parse_expression(None, Precedence::Lowest)?;
                if !matches!(self.next_token, Some(Token::Semicolon)) {
                    return Err("Parsing Error: no semicolon found in Return Statement");
                }
                self.read_next_token();
                Ok(Statement::Return(exp))
            }
            _ => {
                let exp = self.parse_expression(None, Precedence::Lowest)?;
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
        let val = self.parse_expression(None, Precedence::Lowest)?;
        if !matches!(self.next_token, Some(Token::Semicolon)) {
            return Err("Parsing Error: no semicolon found in Let Statement");
        }
        Ok(LetStatement {
            name: ident,
            value: val,
        })
    }

    fn infix_helper(
        &mut self,
        expr: Expression,
        precedence: Precedence,
    ) -> Result<Expression, &'static str> {
        if self
            .next_token
            .as_ref()
            .is_some_and(|tok| tok.is_infix_operator())
        {
            Ok(self.parse_expression(Some(expr), precedence)?)
        } else {
            Ok(expr)
        }
    }

    fn parse_expression(
        &mut self,
        left_expr: Option<Expression>,
        precedence: Precedence,
    ) -> Result<Expression, &'static str> {
        let next_token = self.get_next_token()?;
        match left_expr {
            None => match next_token {
                Token::Int(num) => {
                    let num = *num;
                    self.read_next_token();
                    self.infix_helper(Expression::Int(num), precedence)
                }
                Token::Ident(var) => {
                    let var = var.clone();
                    self.read_next_token();
                    self.infix_helper(Expression::Ident(var), precedence)
                }
                Token::True => {
                    self.read_next_token();
                    self.infix_helper(Expression::Bool(true), precedence)
                }
                Token::False => {
                    self.read_next_token();
                    self.infix_helper(Expression::Bool(false), precedence)
                }
                Token::Bang | Token::Minus => {
                    let operator = next_token.clone();
                    self.read_next_token();
                    let exp = self.parse_expression(None, Precedence::Prefix)?;
                    self.infix_helper(
                        Expression::Prefix(PrefixExpression {
                            operator,
                            exp: Box::new(exp),
                        }),
                        precedence,
                    )
                }
                Token::LParen => {
                    self.read_next_token();
                    let exp = self.parse_expression(None, Precedence::Lowest)?;
                    if *self.get_next_token()? != Token::RParen {
                        return Err("Expected RParen but it's missing");
                    } else {
                        self.read_next_token();
                    }
                    self.infix_helper(exp, precedence)
                }
                _ => Err("Parsing Error: Invalid Expression"),
            },
            Some(left_exp) => {
                if next_token.is_infix_operator() {
                    let op_precedence = Precedence::precedence_of_infix_operator(next_token);
                    if op_precedence > precedence {
                        let operator = next_token.clone();
                        self.read_next_token();
                        let right_exp = Box::new(self.parse_expression(None, op_precedence)?);
                        self.infix_helper(
                            Expression::Infix(InfixExpression {
                                left_exp: Box::new(left_exp),
                                operator,
                                right_exp,
                            }),
                            precedence,
                        )
                    } else {
                        Ok(left_exp)
                    }
                } else {
                    Err("Parsing Error: Invalid Expression")
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        lexer::{Lexer, token::Token},
        parser::{
            Parser,
            ast::{Expression, InfixExpression, LetStatement, PrefixExpression, Statement},
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

    #[test]
    fn test_infix_expressions() {
        let input = "
            5+5;
            5-5;
            5*5;
            5/5;
            5>5;
            5<5;
            5==5;
            5!=5;";
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
            8,
            "program doesn't contain 8 statements. got: {}",
            program.len()
        );
        let mut program = program.iter();
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Int(5)),
                operator: Token::Plus,
                right_exp: Box::new(Expression::Int(5))
            })))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Int(5)),
                operator: Token::Minus,
                right_exp: Box::new(Expression::Int(5))
            })))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Int(5)),
                operator: Token::Asterisk,
                right_exp: Box::new(Expression::Int(5))
            })))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Int(5)),
                operator: Token::Slash,
                right_exp: Box::new(Expression::Int(5))
            })))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Int(5)),
                operator: Token::GT,
                right_exp: Box::new(Expression::Int(5))
            })))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Int(5)),
                operator: Token::LT,
                right_exp: Box::new(Expression::Int(5))
            })))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Int(5)),
                operator: Token::Eq,
                right_exp: Box::new(Expression::Int(5))
            })))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Int(5)),
                operator: Token::NEq,
                right_exp: Box::new(Expression::Int(5))
            })))
        );
        assert_eq!(program.next(), None);
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let input = "
            -a*b
            !-a
            a+b-c
            a*b/c
            a+b/c
            a + b * c + d / e - f
            5 > 4 == 3 < 4
            5 < 4 != 3 > 4
            3 + 4 * 5 == 3 * 1 + 4 * 5";
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
            9,
            "program doesn't contain 9 statements. got: {}",
            program.len()
        );
        let mut program = program.iter();
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Prefix(PrefixExpression {
                    operator: Token::Minus,
                    exp: Box::new(Expression::Ident("a".to_string()))
                })),
                operator: Token::Asterisk,
                right_exp: Box::new(Expression::Ident("b".to_string()))
            })))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Prefix(PrefixExpression {
                operator: Token::Bang,
                exp: Box::new(Expression::Prefix(PrefixExpression {
                    operator: Token::Minus,
                    exp: Box::new(Expression::Ident("a".to_string()))
                }))
            }))),
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Infix(InfixExpression {
                    left_exp: Box::new(Expression::Ident("a".to_string())),
                    operator: Token::Plus,
                    right_exp: Box::new(Expression::Ident("b".to_string()))
                })),
                operator: Token::Minus,
                right_exp: Box::new(Expression::Ident("c".to_string()))
            })))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Infix(InfixExpression {
                    left_exp: Box::new(Expression::Ident("a".to_string())),
                    operator: Token::Asterisk,
                    right_exp: Box::new(Expression::Ident("b".to_string()))
                })),
                operator: Token::Slash,
                right_exp: Box::new(Expression::Ident("c".to_string()))
            })))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Ident("a".to_string())),
                operator: Token::Plus,
                right_exp: Box::new(Expression::Infix(InfixExpression {
                    left_exp: Box::new(Expression::Ident("b".to_string())),
                    operator: Token::Slash,
                    right_exp: Box::new(Expression::Ident("c".to_string()))
                })),
            })))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Infix(InfixExpression {
                    left_exp: Box::new(Expression::Infix(InfixExpression {
                        left_exp: Box::new(Expression::Ident("a".to_string())),
                        operator: Token::Plus,
                        right_exp: Box::new(Expression::Infix(InfixExpression {
                            left_exp: Box::new(Expression::Ident("b".to_string())),
                            operator: Token::Asterisk,
                            right_exp: Box::new(Expression::Ident("c".to_string()))
                        })),
                    })),
                    operator: Token::Plus,
                    right_exp: Box::new(Expression::Infix(InfixExpression {
                        left_exp: Box::new(Expression::Ident("d".to_string())),
                        operator: Token::Slash,
                        right_exp: Box::new(Expression::Ident("e".to_string()))
                    })),
                })),
                operator: Token::Minus,
                right_exp: Box::new(Expression::Ident("f".to_string())),
            })))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Infix(InfixExpression {
                    left_exp: Box::new(Expression::Int(5)),
                    operator: Token::GT,
                    right_exp: Box::new(Expression::Int(4))
                })),
                operator: Token::Eq,
                right_exp: Box::new(Expression::Infix(InfixExpression {
                    left_exp: Box::new(Expression::Int(3)),
                    operator: Token::LT,
                    right_exp: Box::new(Expression::Int(4))
                })),
            })))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Infix(InfixExpression {
                    left_exp: Box::new(Expression::Int(5)),
                    operator: Token::LT,
                    right_exp: Box::new(Expression::Int(4))
                })),
                operator: Token::NEq,
                right_exp: Box::new(Expression::Infix(InfixExpression {
                    left_exp: Box::new(Expression::Int(3)),
                    operator: Token::GT,
                    right_exp: Box::new(Expression::Int(4))
                })),
            })))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Infix(InfixExpression {
                    left_exp: Box::new(Expression::Int(3)),
                    operator: Token::Plus,
                    right_exp: Box::new(Expression::Infix(InfixExpression {
                        left_exp: Box::new(Expression::Int(4)),
                        operator: Token::Asterisk,
                        right_exp: Box::new(Expression::Int(5))
                    }))
                })),
                operator: Token::Eq,
                right_exp: Box::new(Expression::Infix(InfixExpression {
                    left_exp: Box::new(Expression::Infix(InfixExpression {
                        left_exp: Box::new(Expression::Int(3)),
                        operator: Token::Asterisk,
                        right_exp: Box::new(Expression::Int(1))
                    })),
                    operator: Token::Plus,
                    right_exp: Box::new(Expression::Infix(InfixExpression {
                        left_exp: Box::new(Expression::Int(4)),
                        operator: Token::Asterisk,
                        right_exp: Box::new(Expression::Int(5))
                    }))
                })),
            })))
        );
        assert_eq!(program.next(), None);
    }

    #[test]
    fn test_boolean_literal_expression() {
        let input = "true;false;";
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
            Some(&Statement::Expr(Expression::Bool(true)))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Bool(false)))
        );
        assert_eq!(program.next(), None);
    }

    #[test]
    fn test_operator_precedence_parsing_with_parantheses() {
        let input = "
            -( a*b )
            !( -a )
            a+( b-c )
            a*( b/c )
            ( a+b )/c
            ( a + b ) * ( c + d ) / ( e - f )
            ( 3 + 4 ) * ( ( 5 == 3 ) * ( 1 + 4 ) * 5 )";
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
            7,
            "program doesn't contain 7 statements. got: {}",
            program.len()
        );
        let mut program = program.iter();
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Prefix(PrefixExpression {
                operator: Token::Minus,
                exp: Box::new(Expression::Infix(InfixExpression {
                    left_exp: Box::new(Expression::Ident("a".to_string())),
                    operator: Token::Asterisk,
                    right_exp: Box::new(Expression::Ident("b".to_string()))
                })),
            })))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Prefix(PrefixExpression {
                operator: Token::Bang,
                exp: Box::new(Expression::Prefix(PrefixExpression {
                    operator: Token::Minus,
                    exp: Box::new(Expression::Ident("a".to_string()))
                }))
            }))),
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Ident("a".to_string())),
                operator: Token::Plus,
                right_exp: Box::new(Expression::Infix(InfixExpression {
                    left_exp: Box::new(Expression::Ident("b".to_string())),
                    operator: Token::Minus,
                    right_exp: Box::new(Expression::Ident("c".to_string()))
                })),
            })))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Ident("a".to_string())),
                operator: Token::Asterisk,
                right_exp: Box::new(Expression::Infix(InfixExpression {
                    left_exp: Box::new(Expression::Ident("b".to_string())),
                    operator: Token::Slash,
                    right_exp: Box::new(Expression::Ident("c".to_string()))
                })),
            })))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Infix(InfixExpression {
                    left_exp: Box::new(Expression::Ident("a".to_string())),
                    operator: Token::Plus,
                    right_exp: Box::new(Expression::Ident("b".to_string()))
                })),
                operator: Token::Slash,
                right_exp: Box::new(Expression::Ident("c".to_string())),
            })))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Infix(InfixExpression {
                    left_exp: Box::new(Expression::Infix(InfixExpression {
                        left_exp: Box::new(Expression::Ident("a".to_string())),
                        operator: Token::Plus,
                        right_exp: Box::new(Expression::Ident("b".to_string())),
                    })),
                    operator: Token::Asterisk,
                    right_exp: Box::new(Expression::Infix(InfixExpression {
                        left_exp: Box::new(Expression::Ident("c".to_string())),
                        operator: Token::Plus,
                        right_exp: Box::new(Expression::Ident("d".to_string()))
                    })),
                })),
                operator: Token::Slash,
                right_exp: Box::new(Expression::Infix(InfixExpression {
                    left_exp: Box::new(Expression::Ident("e".to_string())),
                    operator: Token::Minus,
                    right_exp: Box::new(Expression::Ident("f".to_string()))
                })),
            })))
        );
        assert_eq!(
            program.next(),
            Some(&Statement::Expr(Expression::Infix(InfixExpression {
                left_exp: Box::new(Expression::Infix(InfixExpression {
                    left_exp: Box::new(Expression::Int(3)),
                    operator: Token::Plus,
                    right_exp: Box::new(Expression::Int(4))
                })),
                operator: Token::Asterisk,
                right_exp: Box::new(Expression::Infix(InfixExpression {
                    left_exp: Box::new(Expression::Infix(InfixExpression {
                        left_exp: Box::new(Expression::Infix(InfixExpression {
                            left_exp: Box::new(Expression::Int(5)),
                            operator: Token::Eq,
                            right_exp: Box::new(Expression::Int(3))
                        })),
                        operator: Token::Asterisk,
                        right_exp: Box::new(Expression::Infix(InfixExpression {
                            left_exp: Box::new(Expression::Int(1)),
                            operator: Token::Plus,
                            right_exp: Box::new(Expression::Int(4)),
                        }))
                    })),
                    operator: Token::Asterisk,
                    right_exp: Box::new(Expression::Int(5))
                })),
            })))
        );
        assert_eq!(program.next(), None);
    }
}
