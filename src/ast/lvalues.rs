use crate::{
    ast::{AstNode, BinaryAstNode},
    compiler::NodeCompiler,
};
use std::fmt::Display;

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
    fn accept(&self, visitor: &dyn NodeCompiler) -> String {
        visitor.compile_assignment(self)
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Assignment({}, {})", self.left, self.right)
    }
}

impl AstNode for Identifier {
    fn accept(&self, visitor: &dyn NodeCompiler) -> String {
        visitor.compile_identifier(self)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
