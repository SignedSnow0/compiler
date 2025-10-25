use std::fmt::Display;

use crate::{ast::AstNode, compiler::NodeCompiler};

pub trait RValue: AstNode {
    fn default() -> Box<dyn AstNode>;
}

pub struct Integer {
    pub value: i32,
}

impl RValue for Integer {
    fn default() -> Box<dyn AstNode> {
        Box::new(Self { value: 0 })
    }
}

impl AstNode for Integer {
    fn accept(&self, visitor: &dyn NodeCompiler) -> String {
        visitor.compile_int_lit(self)
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
