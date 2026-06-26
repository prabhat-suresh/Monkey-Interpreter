use crate::{
    evaluator::object::Object,
    parser::{
        Program,
        ast::{Expression, Statement},
    },
};

pub mod object;

pub fn eval(prog: Program) -> Option<Object> {
    prog.iter().map(|stmt| eval_statement(stmt)).last()
}

fn eval_statement(stmt: &Statement) -> Object {
    match stmt {
        Statement::Expr(expr) => eval_expression(expr),
        _ => Object::Null,
    }
}

fn eval_expression(expr: &Expression) -> Object {
    match expr {
        Expression::Int(n) => Object::Int(*n as i64),
        Expression::Bool(b) => Object::Bool(*b),
        _ => Object::Null,
    }
}

#[cfg(test)]
mod test {
    use crate::{
        evaluator::{eval, object::Object},
        lexer::Lexer,
        parser::Parser,
    };

    fn test_eval(input: &str) -> Result<Option<Object>, &'static str> {
        let l = Lexer::new(input.bytes());
        let mut p = Parser::new(l);
        Ok(eval(p.parse_program()?))
    }
    #[test]
    fn eval_integer_expression() {
        let input = "
            5
            10";
        assert!(test_eval(input).is_ok_and(|obj| obj == Some(Object::Int(10))));
    }
    #[test]
    fn eval_boolean_expression() {
        let input = "
            true
            false";
        assert!(test_eval(input).is_ok_and(|obj| obj == Some(Object::Bool(false))));
    }
}
