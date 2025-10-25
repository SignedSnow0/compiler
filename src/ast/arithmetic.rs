use std::fmt::Display;

use crate::ast::{AstNode, BinaryAstNode};

pub struct Addition {
    pub left: Box<dyn AstNode<TEval = i32>>,
    pub right: Box<dyn AstNode<TEval = i32>>,
}

pub struct Subtraction {
    pub left: Box<dyn AstNode<TEval = i32>>,
    pub right: Box<dyn AstNode<TEval = i32>>,
}

pub struct Multiplication {
    pub left: Box<dyn AstNode<TEval = i32>>,
    pub right: Box<dyn AstNode<TEval = i32>>,
}

pub struct Division {
    pub left: Box<dyn AstNode<TEval = i32>>,
    pub right: Box<dyn AstNode<TEval = i32>>,
}

impl BinaryAstNode for Addition {
    fn new(
        left: Box<dyn AstNode<TEval = Self::TEval>>,
        right: Box<dyn AstNode<TEval = Self::TEval>>,
    ) -> Box<dyn BinaryAstNode<TEval = Self::TEval>>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Addition {
    type TEval = i32;

    fn eval(&self) -> Self::TEval {
        self.left.eval() + self.right.eval()
    }
}

impl BinaryAstNode for Subtraction {
    fn new(
        left: Box<dyn AstNode<TEval = Self::TEval>>,
        right: Box<dyn AstNode<TEval = Self::TEval>>,
    ) -> Box<dyn BinaryAstNode<TEval = Self::TEval>>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Subtraction {
    type TEval = i32;

    fn eval(&self) -> Self::TEval {
        self.left.eval() - self.right.eval()
    }
}

impl BinaryAstNode for Multiplication {
    fn new(
        left: Box<dyn AstNode<TEval = Self::TEval>>,
        right: Box<dyn AstNode<TEval = Self::TEval>>,
    ) -> Box<dyn BinaryAstNode<TEval = Self::TEval>>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Multiplication {
    type TEval = i32;

    fn eval(&self) -> Self::TEval {
        self.left.eval() * self.right.eval()
    }
}

impl BinaryAstNode for Division {
    fn new(
        left: Box<dyn AstNode<TEval = Self::TEval>>,
        right: Box<dyn AstNode<TEval = Self::TEval>>,
    ) -> Box<dyn BinaryAstNode<TEval = Self::TEval>>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Division {
    type TEval = i32;

    fn eval(&self) -> Self::TEval {
        self.left.eval() / self.right.eval()
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
