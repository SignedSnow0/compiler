use std::fmt::Display;

use crate::{
    ast::{AstNode, BinaryAstNode},
    compiler::NodeCompiler,
};

pub struct Addition {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

pub struct Subtraction {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

pub struct Multiplication {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

pub struct Division {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

impl BinaryAstNode for Addition {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn BinaryAstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Addition {
    fn accept(&self, visitor: &dyn NodeCompiler) -> String {
        visitor.compile_sum(self)
    }
}

impl BinaryAstNode for Subtraction {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn BinaryAstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Subtraction {
    fn accept(&self, visitor: &dyn NodeCompiler) -> String {
        visitor.compile_subtraction(self)
    }
}

impl BinaryAstNode for Multiplication {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn BinaryAstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Multiplication {
    fn accept(&self, visitor: &dyn NodeCompiler) -> String {
        visitor.compile_multiplication(self)
    }
}

impl BinaryAstNode for Division {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn BinaryAstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Division {
    fn accept(&self, visitor: &dyn NodeCompiler) -> String {
        visitor.compile_division(self)
    }
}

impl Display for Addition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Addition({}, {})", self.left, self.right)
    }
}

impl Display for Subtraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Subtraction({}, {})", self.left, self.right)
    }
}

impl Display for Multiplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Multiplication({}, {})", self.left, self.right)
    }
}

impl Display for Division {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Division({}, {})", self.left, self.right)
    }
}
