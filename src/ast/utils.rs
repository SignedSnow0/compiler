use crate::{
    ast::{AstNode, Identifier, Integer, LiteralAstNode},
    compiler::AstVisitor,
};
use anyhow::Result;

impl Identifier {
    pub fn new(name: String) -> Box<Identifier> {
        Box::new(Self { name })
    }
}

impl AstNode for Identifier {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_identifier(self)
    }
}

impl LiteralAstNode<i32> for Integer {
    fn new(value: i32) -> Box<Integer>
    where
        Self: Sized,
    {
        Box::new(Self { value })
    }
}

impl AstNode for Integer {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_integer(self)
    }
}
