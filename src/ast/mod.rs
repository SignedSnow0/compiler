pub mod arithmetic;
pub mod boolean;
pub mod rvalues;

use std::fmt::Display;

pub trait AstNode: Display {
    type TEval;

    fn eval(&self) -> Self::TEval;
}

pub trait BinaryAstNode: AstNode {
    fn new(
        left: Box<dyn AstNode<TEval = Self::TEval>>,
        right: Box<dyn AstNode<TEval = Self::TEval>>,
    ) -> Box<dyn BinaryAstNode<TEval = Self::TEval>>
    where
        Self: Sized;
}
