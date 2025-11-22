use crate::{
    ast::{
        Addition, And, AstNode, BinaryAstNode, Division, Equality, Greater, GreaterEqual,
        Inequality, Lesser, LesserEqual, Multiplication, Or, Subtraction,
    },
    compiler::AstVisitor,
};
use anyhow::Result;

impl BinaryAstNode for Or {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn AstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Or {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_or(self)
    }
}

impl BinaryAstNode for And {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn AstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for And {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_and(self)
    }
}

impl BinaryAstNode for Equality {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn AstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Equality {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_equality(self)
    }
}

impl BinaryAstNode for Inequality {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn AstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Inequality {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_inequality(self)
    }
}

impl BinaryAstNode for Greater {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn AstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Greater {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_greater(self)
    }
}

impl BinaryAstNode for Lesser {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn AstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Lesser {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_lesser(self)
    }
}

impl BinaryAstNode for GreaterEqual {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn AstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for GreaterEqual {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_greater_equal(self)
    }
}

impl BinaryAstNode for LesserEqual {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn AstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for LesserEqual {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_lesser_equal(self)
    }
}

impl BinaryAstNode for Addition {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn AstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Addition {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_addition(self)
    }
}

impl BinaryAstNode for Subtraction {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn AstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Subtraction {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_subtraction(self)
    }
}

impl BinaryAstNode for Multiplication {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn AstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Multiplication {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_multiplication(self)
    }
}

impl BinaryAstNode for Division {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn AstNode>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Division {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_division(self)
    }
}
