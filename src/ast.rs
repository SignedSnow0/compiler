use std::fmt::Display;

pub trait AstNode: Display {
    type TEval;

    fn eval(&self) -> Self::TEval;
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

impl AstNode for Addition {
    type TEval = i32;

    fn eval(&self) -> Self::TEval {
        self.left.eval() + self.right.eval()
    }
}

impl AstNode for Subtraction {
    type TEval = i32;

    fn eval(&self) -> Self::TEval {
        self.left.eval() - self.right.eval()
    }
}

impl AstNode for Multiplication {
    type TEval = i32;

    fn eval(&self) -> Self::TEval {
        self.left.eval() * self.right.eval()
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
