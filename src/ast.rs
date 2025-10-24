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

pub struct LogicalNot {
    pub value: Box<dyn AstNode<TEval = i32>>,
}

pub struct LogicalOr {
    pub left: Box<dyn AstNode<TEval = i32>>,
    pub right: Box<dyn AstNode<TEval = i32>>,
}

pub struct LogicalAnd {
    pub left: Box<dyn AstNode<TEval = i32>>,
    pub right: Box<dyn AstNode<TEval = i32>>,
}

pub struct Less {
    pub left: Box<dyn AstNode<TEval = i32>>,
    pub right: Box<dyn AstNode<TEval = i32>>,
}

pub struct Greater {
    pub left: Box<dyn AstNode<TEval = i32>>,
    pub right: Box<dyn AstNode<TEval = i32>>,
}

pub struct LessEqual {
    pub left: Box<dyn AstNode<TEval = i32>>,
    pub right: Box<dyn AstNode<TEval = i32>>,
}

pub struct GreaterEqual {
    pub left: Box<dyn AstNode<TEval = i32>>,
    pub right: Box<dyn AstNode<TEval = i32>>,
}

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

pub struct Integer {
    pub value: i32,
}

impl AstNode for LogicalNot {
    type TEval = i32;

    fn eval(&self) -> Self::TEval {
        let value = self.value.eval() != 0;
        (!value).into()
    }
}

impl BinaryAstNode for LogicalOr {
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

impl AstNode for LogicalOr {
    type TEval = i32;

    fn eval(&self) -> Self::TEval {
        let left = self.left.eval() != 0;
        let right = self.right.eval() != 0;
        (left || right).into()
    }
}

impl BinaryAstNode for LogicalAnd {
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

impl AstNode for LogicalAnd {
    type TEval = i32;

    fn eval(&self) -> Self::TEval {
        let left = self.left.eval() != 0;
        let right = self.right.eval() != 0;
        (left && right).into()
    }
}

impl BinaryAstNode for Less {
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

impl AstNode for Less {
    type TEval = i32;

    fn eval(&self) -> Self::TEval {
        (self.left.eval() < self.right.eval()).into()
    }
}

impl BinaryAstNode for Greater {
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

impl AstNode for Greater {
    type TEval = i32;

    fn eval(&self) -> Self::TEval {
        (self.left.eval() > self.right.eval()).into()
    }
}

impl BinaryAstNode for LessEqual {
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

impl AstNode for LessEqual {
    type TEval = i32;

    fn eval(&self) -> Self::TEval {
        (self.left.eval() <= self.right.eval()).into()
    }
}

impl BinaryAstNode for GreaterEqual {
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

impl AstNode for GreaterEqual {
    type TEval = i32;

    fn eval(&self) -> Self::TEval {
        (self.left.eval() >= self.right.eval()).into()
    }
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

impl AstNode for Integer {
    type TEval = i32;

    fn eval(&self) -> Self::TEval {
        self.value
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

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
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
