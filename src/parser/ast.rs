use crate::lexer::token::Token;

pub type Program = Vec<Statement>;
pub type BlockStatement = Vec<Statement>;

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
    Bool(bool),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    IfElse(IfElseExpression),
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

#[derive(Debug, PartialEq)]
pub struct InfixExpression {
    pub left_exp: Box<Expression>,
    pub operator: Token,
    pub right_exp: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct IfElseExpression {
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: BlockStatement,
}

pub type Identifier = String;
