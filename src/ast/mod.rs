use crate::compiler::AstVisitor;
use anyhow::Result;
use std::collections::HashMap;

pub mod declarations;
pub mod instructions;
pub mod operators;
pub mod utils;

pub trait AstNode {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()>;
}

pub trait LiteralAstNode<T> {
    fn new(value: T) -> Box<Self>
    where
        Self: Sized;
}

pub trait BinaryAstNode: AstNode {
    fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Box<Self>
    where
        Self: Sized;
}

pub struct Program {
    pub nodes: Vec<Box<dyn AstNode>>,
}

pub struct StructTypedef {
    pub name: String,
    pub fields: HashMap<String, Type>,
}

pub struct Declaration {
    pub name: String,
    pub d_type: Option<Type>,
    pub value: Option<Box<dyn AstNode>>,
}

pub struct Function {
    pub name: String,
    pub parameters: HashMap<String, Type>,
    pub return_type: Option<Type>,
    pub body: Box<dyn AstNode>,
}

pub struct Block {
    pub nodes: Vec<Box<dyn AstNode>>,
}

pub struct Assignment {
    pub target: String,
    pub value: Box<dyn AstNode>,
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

pub struct Or {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

pub struct And {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

pub struct Equality {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

pub struct Inequality {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

pub struct Lesser {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

pub struct Greater {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

pub struct LesserEqual {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

pub struct GreaterEqual {
    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,
}

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

pub struct Identifier {
    pub name: String,
}

pub struct Integer {
    pub value: i32,
}

pub struct Character {
    pub value: char,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Type {
    Integer32,
    Boolean8,
    Char8,
    Custom(String),
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
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit(self)
    }
}
