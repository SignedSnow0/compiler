use crate::{
    ast::{AstNode, BinaryAstNode},
    compiler::NodeCompiler,
};
use anyhow::Result;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct Identifier {
    pub name: String,
}

pub struct Assignment {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

impl BinaryAstNode for Assignment {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn BinaryAstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Assignment {
    fn accept(&self, visitor: &mut dyn NodeCompiler) -> Result<()> {
        visitor.compile_assignment(self)
    }
}

impl AstNode for Identifier {
    fn accept(&self, visitor: &mut dyn NodeCompiler) -> Result<()> {
        visitor.compile_identifier(self)
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Assignment({}, {})", self.left, self.right)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.name)
    }
}
