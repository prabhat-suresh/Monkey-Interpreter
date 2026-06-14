use crate::lexer::token::Token;

pub type Program = Vec<Statement>;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Return(Expression),
    Expr(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Ident(String),
    Int(i32),
    Prefix(PrefixExpression),
}

#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct PrefixExpression {
    pub operator: Token,
    pub exp: Box<Expression>,
}

pub type Identifier = String;
