use std::fmt::Display;

use crate::ast::AstNode;

pub struct Integer {
    pub value: i32,
}

impl AstNode for Integer {
    type TEval = i32;

    fn eval(&self) -> Self::TEval {
        self.value
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
