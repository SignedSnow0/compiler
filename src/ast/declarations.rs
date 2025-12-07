use crate::{
    ast::{AstNode, Declaration, Function, StructTypedef, Type},
    compiler::AstVisitor,
};
use anyhow::Result;
use std::collections::HashMap;

impl StructTypedef {
    pub fn new(name: String, fields: HashMap<String, Type>) -> Box<StructTypedef> {
        Box::new(Self { name, fields })
    }
}

impl AstNode for StructTypedef {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_typedef(self)
    }
}

impl Declaration {
    pub fn new(
        name: String,
        d_type: Option<Type>,
        value: Option<Box<dyn AstNode>>,
    ) -> Box<Declaration> {
        Box::new(Self {
            name,
            d_type,
            value,
        })
    }
}

impl AstNode for Declaration {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_declaration(self)
    }
}

impl Function {
    pub fn new(
        name: String,
        parameters: HashMap<String, Type>,
        return_type: Option<Type>,
        body: Box<dyn AstNode>,
    ) -> Box<Function> {
        Box::new(Self {
            name,
            parameters,
            return_type,
            body,
        })
    }
}

impl AstNode for Function {
    fn accept(&mut self, visitor: &mut dyn AstVisitor) -> Result<()> {
        visitor.visit_function(self)
    }
}
