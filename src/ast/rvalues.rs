use std::fmt::{Display, Formatter, Result as FmtResult};
use anyhow::Result;
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
    fn accept(&self, visitor: &mut dyn NodeCompiler) -> Result<()> {
        visitor.compile_int_lit(self)
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.value)
    }
}
