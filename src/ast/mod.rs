use crate::compiler::AstVisitor;
use anyhow::Result;

pub trait AstNode {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()>;
}

pub trait LiteralAstNode<T> {
    fn new(value: T) -> Box<dyn AstNode>
    where
        Self: Sized;
}

pub trait BinaryAstNode: AstNode {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<dyn AstNode>
    where
        Self: Sized;
}

pub struct Or {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

pub struct And {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

pub struct Greater {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

pub struct Lesser {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

pub struct GreaterEqual {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

pub struct LesserEqual {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

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

pub struct Program {
    pub nodes: Vec<Box<dyn AstNode>>,
}

impl Program {
    pub fn new() -> Box<Program> {
        Box::new(Self { nodes: vec![] })
    }

    pub fn add_node(&mut self, node: Box<dyn AstNode>) {
        self.nodes.push(node);
    }
}

impl AstNode for Program {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit(self)
    }
}

pub struct Identifier {
    pub name: String,
}

pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: String,
    pub body: Box<dyn AstNode>,
}

pub struct If {
    pub condition: Box<dyn AstNode>,
    pub then_block: Box<dyn AstNode>,
    pub else_block: Option<Box<dyn AstNode>>,
}

pub struct While {
    pub condition: Box<dyn AstNode>,
    pub block: Box<dyn AstNode>,
}

pub struct Return {
    pub value: Box<dyn AstNode>,
}

pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Box<dyn AstNode>>,
}

impl Return {
    pub fn new(value: Box<dyn AstNode>) -> Box<dyn AstNode> {
        Box::new(Self { value })
    }
}

impl AstNode for Return {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_return(self)
    }
}

impl FunctionCall {
    pub fn new(name: String, arguments: Vec<Box<dyn AstNode>>) -> Box<dyn AstNode> {
        Box::new(Self { name, arguments })
    }
}

impl AstNode for FunctionCall {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_function_call(self)
    }
}

impl Identifier {
    pub fn new(name: String) -> Box<dyn AstNode> {
        Box::new(Self { name })
    }
}

impl AstNode for Identifier {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_identifier(&self)
    }
}

impl While {
    pub fn new(condition: Box<dyn AstNode>, block: Box<dyn AstNode>) -> Box<dyn AstNode> {
        Box::new(Self { condition, block })
    }
}

impl AstNode for While {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_while(self)
    }
}

impl If {
    pub fn new(
        condition: Box<dyn AstNode>,
        then_block: Box<dyn AstNode>,
        else_block: Option<Box<dyn AstNode>>,
    ) -> Box<dyn AstNode> {
        Box::new(Self {
            condition,
            then_block,
            else_block,
        })
    }
}

impl AstNode for If {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_if(self)
    }
}

impl Function {
    pub fn new(
        name: String,
        parameters: Vec<Parameter>,
        return_type: String,
        body: Box<dyn AstNode>,
    ) -> Box<dyn AstNode> {
        Box::new(Self {
            name,
            parameters,
            return_type,
            body,
        })
    }
}

impl AstNode for Function {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_function(self)
    }
}

pub struct Assignment {
    pub target: String,
    pub value: Box<dyn AstNode>,
}

impl Assignment {
    pub fn new(target: String, value: Box<dyn AstNode>) -> Box<dyn AstNode> {
        Box::new(Self { target, value })
    }
}

impl AstNode for Assignment {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_assignment(&self)
    }
}

pub struct Parameter {
    pub name: String,
    pub type_name: String,
}

pub struct Integer {
    pub value: i32,
}

pub struct Declaration {
    pub name: String,
    pub type_name: String,
    pub value: Box<dyn AstNode>,
}

impl Declaration {
    pub fn new(name: String, type_name: String, value: Box<dyn AstNode>) -> Box<dyn AstNode> {
        Box::new(Self {
            name,
            type_name,
            value,
        })
    }
}

impl AstNode for Declaration {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_declaration(&self)
    }
}

impl LiteralAstNode<i32> for Integer {
    fn new(value: i32) -> Box<dyn AstNode>
    where
        Self: Sized,
    {
        Box::new(Self { value })
    }
}

impl AstNode for Integer {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_integer(self)
    }
}

pub struct Addition {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
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

pub struct Subtraction {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
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

pub struct Multiplication {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
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

pub struct Division {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
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

pub struct Block {
    pub nodes: Vec<Box<dyn AstNode>>,
}

impl Block {
    pub fn new() -> Box<Block> {
        Box::new(Self { nodes: vec![] })
    }

    pub fn add_node(&mut self, node: Box<dyn AstNode>) {
        self.nodes.push(node);
    }
}

impl AstNode for Block {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_block(self)
    }
}
