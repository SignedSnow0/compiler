use std::fmt::Display;

use crate::{
    ast::{AstNode, BinaryAstNode},
    compiler::NodeCompiler,
};

pub struct LogicalNot {
    pub value: Box<dyn AstNode>,
}

pub struct LogicalOr {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

pub struct LogicalAnd {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

pub struct Less {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

pub struct Greater {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

pub struct LessEqual {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

pub struct GreaterEqual {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

impl AstNode for LogicalNot {
    fn accept(&self, visitor: &dyn NodeCompiler) -> String {
        visitor.compile_logical_not(self)
    }
}

impl BinaryAstNode for LogicalOr {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn BinaryAstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for LogicalOr {
    fn accept(&self, visitor: &dyn NodeCompiler) -> String {
        visitor.compile_logical_or(self)
    }
}

impl BinaryAstNode for LogicalAnd {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn BinaryAstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for LogicalAnd {
    fn accept(&self, visitor: &dyn NodeCompiler) -> String {
        visitor.compile_logical_and(self)
    }
}

impl BinaryAstNode for Less {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn BinaryAstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Less {
    fn accept(&self, visitor: &dyn NodeCompiler) -> String {
        visitor.compile_less(self)
    }
}

impl BinaryAstNode for Greater {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn BinaryAstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Greater {
    fn accept(&self, visitor: &dyn NodeCompiler) -> String {
        visitor.compile_greater(self)
    }
}

impl BinaryAstNode for LessEqual {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn BinaryAstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for LessEqual {
    fn accept(&self, visitor: &dyn NodeCompiler) -> String {
        visitor.compile_less_equal(self)
    }
}

impl BinaryAstNode for GreaterEqual {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn BinaryAstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for GreaterEqual {
    fn accept(&self, visitor: &dyn NodeCompiler) -> String {
        visitor.compile_greater_equal(self)
    }
}

impl Display for Less {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Less({}, {})", self.left, self.right)
    }
}

impl Display for Greater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Greater({}, {})", self.left, self.right)
    }
}

impl Display for LessEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LessEqual({}, {})", self.left, self.right)
    }
}

impl Display for GreaterEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GreaterEqual({}, {})", self.left, self.right)
    }
}

impl Display for LogicalNot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not({})", self.value)
    }
}

impl Display for LogicalOr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Or({}, {})", self.left, self.right)
    }
}

impl Display for LogicalAnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "And({}, {})", self.left, self.right)
    }
}
