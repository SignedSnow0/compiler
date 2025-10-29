pub mod arithmetic;
pub mod boolean;
pub mod control_flow;
pub mod lvalues;
pub mod rvalues;

use std::fmt::Display;

use crate::compiler::NodeCompiler;
use anyhow::Result;

pub trait AstNode: Display {
    fn accept(&self, visitor: &mut dyn NodeCompiler) -> Result<()>;
}

pub trait BinaryAstNode: AstNode {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn BinaryAstNode>
    where
        Self: Sized;
}

pub trait GenericAstNode: AstNode {
    fn new() -> Box<dyn GenericAstNode>
    where
        Self: Sized;
    fn add_node(&mut self, node: Box<dyn AstNode>);
}
