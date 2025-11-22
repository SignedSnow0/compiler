use crate::{
    ast::{AstNode, Declaration, Function, Parameter, Typedef},
    compiler::AstVisitor,
};
use anyhow::Result;

impl Typedef {
    pub fn new(name: String, fields: Vec<Parameter>) -> Box<dyn AstNode> {
        Box::new(Self { name, fields })
    }
}

impl AstNode for Typedef {
    fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_typedef(self)
    }
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
