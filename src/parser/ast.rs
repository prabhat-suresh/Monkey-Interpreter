pub type Program = Vec<Statement>;

#[derive(Debug)]
pub enum Node {
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(LetStatement),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Int(i32),
}

#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

pub type Identifier = String;
