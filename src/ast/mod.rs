pub mod arithmetic;
pub mod boolean;
pub mod lvalues;
pub mod rvalues;

use std::fmt::Display;

use crate::compiler::NodeCompiler;

pub trait AstNode: Display {
    fn accept(&self, visitor: &dyn NodeCompiler) -> String;
}

pub trait BinaryAstNode: AstNode {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn BinaryAstNode>
    where
        Self: Sized;
}
