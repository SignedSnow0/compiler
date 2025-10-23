use crate::{Expression, Factor, Term};

pub trait AstNode {
    type TEval;

    fn eval(&self) -> Self::TEval;
}

impl AstNode for Expression {
    type TEval = u32;

    fn eval(&self) -> Self::TEval {
        match self {
            Expression::Addition(left, right) => left.eval() + right.eval(),
            Expression::Subtraction(left, right) => left.eval() - right.eval(),
            Expression::Term(term) => term.eval(),
        }
    }
}

impl AstNode for Term {
    type TEval = u32;

    fn eval(&self) -> Self::TEval {
        match self {
            Term::Multiplication(left, right) => left.eval() * right.eval(),
            Term::Division(left, right) => left.eval() / right.eval(),
            Term::Factor(factor) => factor.eval(),
        }
    }
}

impl AstNode for Factor {
    type TEval = u32;

    fn eval(&self) -> Self::TEval {
        match self {
            Factor::Integer(value) => *value,
            Factor::Expression(expression) => expression.eval(),
        }
    }
}
