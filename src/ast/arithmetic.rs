use std::fmt::Display;
use anyhow::Result;
use std::fmt::{Formatter, Result as FmtResult};

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
    fn accept(&self, visitor: &mut dyn NodeCompiler) -> Result<()> {
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
    fn accept(&self, visitor: &mut dyn NodeCompiler) -> Result<()> {
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
    fn accept(&self, visitor: &mut dyn NodeCompiler) -> Result<()> {
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
    fn accept(&self, visitor: &mut dyn NodeCompiler) -> Result<()> {
        visitor.compile_division(self)
    }
}

impl Display for Addition {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Addition({}, {})", self.left, self.right)
    }
}

impl Display for Subtraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Subtraction({}, {})", self.left, self.right)
    }
}

impl Display for Multiplication {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Multiplication({}, {})", self.left, self.right)
    }
}

impl Display for Division {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Division({}, {})", self.left, self.right)
    }
}
