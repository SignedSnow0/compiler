use crate::{
    ast::{
        Addition, And, AstNode, BinaryAstNode, Division, Equality, Greater, GreaterEqual,
        Inequality, Lesser, LesserEqual, Multiplication, Or, Subtraction,
    },
    compiler::AstVisitor,
};
use anyhow::Result;

impl BinaryAstNode for Or {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<Or>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Or {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_or(self)
    }
}

impl BinaryAstNode for And {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<And>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for And {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_and(self)
    }
}

impl BinaryAstNode for Equality {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<Equality>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Equality {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_equality(self)
    }
}

impl BinaryAstNode for Inequality {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<Inequality>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Inequality {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_inequality(self)
    }
}

impl BinaryAstNode for Greater {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<Greater>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Greater {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_greater(self)
    }
}

impl BinaryAstNode for Lesser {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<Lesser>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Lesser {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_lesser(self)
    }
}

impl BinaryAstNode for GreaterEqual {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<GreaterEqual>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for GreaterEqual {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_greater_equal(self)
    }
}

impl BinaryAstNode for LesserEqual {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<LesserEqual>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for LesserEqual {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_lesser_equal(self)
    }
}

impl BinaryAstNode for Addition {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<Addition>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Addition {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_addition(self)
    }
}

impl BinaryAstNode for Subtraction {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<Subtraction>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Subtraction {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_subtraction(self)
    }
}

impl BinaryAstNode for Multiplication {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<Multiplication>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Multiplication {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_multiplication(self)
    }
}

impl BinaryAstNode for Division {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<Division>
    where
        Self: Sized,
    {
        Box::new(Self { left, right })
    }
}

impl AstNode for Division {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_division(self)
    }
}
