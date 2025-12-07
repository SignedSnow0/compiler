use crate::{
    ast::{Assignment, AstNode, Block, FunctionCall, If, Return, While},
    compiler::AstVisitor,
};
use anyhow::Result;

impl If {
    pub fn new(
        condition: Box<dyn AstNode>,
        then_block: Box<dyn AstNode>,
        else_block: Option<Box<dyn AstNode>>,
    ) -> Box<If> {
        Box::new(Self {
            condition,
            then_block,
            else_block,
        })
    }
}

impl AstNode for If {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_if(self)
    }
}

impl While {
    pub fn new(condition: Box<dyn AstNode>, block: Box<dyn AstNode>) -> Box<While> {
        Box::new(Self { condition, block })
    }
}

impl AstNode for While {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_while(self)
    }
}

impl Return {
    pub fn new(value: Box<dyn AstNode>) -> Box<Return> {
        Box::new(Self { value })
    }
}

impl AstNode for Return {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_return(self)
    }
}

impl FunctionCall {
    pub fn new(name: String, arguments: Vec<Box<dyn AstNode>>) -> Box<FunctionCall> {
        Box::new(Self { name, arguments })
    }
}

impl AstNode for FunctionCall {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_function_call(self)
    }
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
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_block(self)
    }
}

impl Assignment {
    pub fn new(target: String, value: Box<dyn AstNode>) -> Box<Assignment> {
        Box::new(Self { target, value })
    }
}

impl AstNode for Assignment {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_assignment(self)
    }
}
