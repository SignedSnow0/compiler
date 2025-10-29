use crate::{
    ast::{AstNode, GenericAstNode},
    compiler::NodeCompiler,
};
use anyhow::Result;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct Block {
    nodes: Vec<Box<dyn AstNode>>,
}

pub struct If {
    expression: Box<dyn AstNode>,
    then: Box<dyn AstNode>,
    otherwise: Option<Box<dyn AstNode>>,
}

pub struct While {
    expression: Box<dyn AstNode>,
    block: Box<dyn AstNode>,
}

impl If {
    pub fn new(
        expression: Box<dyn AstNode>,
        then: Box<dyn AstNode>,
        otherwise: Option<Box<dyn AstNode>>,
    ) -> Box<dyn AstNode> {
        Box::new(Self {
            expression,
            then,
            otherwise,
        })
    }
}

impl While {
    pub fn new(expression: Box<dyn AstNode>, block: Box<dyn AstNode>) -> Box<dyn AstNode> {
        Box::new(Self { expression, block })
    }
}

impl GenericAstNode for Block {
    fn new() -> Box<dyn GenericAstNode> {
        Box::new(Self { nodes: Vec::new() })
    }

    fn add_node(&mut self, node: Box<dyn AstNode>) {
        self.nodes.push(node);
    }
}

impl AstNode for Block {
    fn accept(&self, visitor: &mut dyn NodeCompiler) -> Result<()> {
        for node in &self.nodes {
            node.accept(visitor)?;
        }
        Ok(())
    }
}

impl AstNode for If {
    fn accept(&self, visitor: &mut dyn NodeCompiler) -> Result<()> {
        self.expression.accept(visitor)?;
        self.then.accept(visitor)?;
        if let Some(expression) = &self.otherwise {
            expression.accept(visitor)?;
        }

        Ok(())
    }
}

impl AstNode for While {
    fn accept(&self, visitor: &mut dyn NodeCompiler) -> Result<()> {
        self.expression.accept(visitor)?;
        self.block.accept(visitor)?;

        Ok(())
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.nodes.is_empty() {
            write!(f, "Block()")
        } else {
            write!(f, "Block(")?;
            for node in &self.nodes[..self.nodes.len() - 1] {
                write!(f, "{}, ", node)?;
            }
            write!(f, "{})", self.nodes.last().unwrap())
        }
    }
}

impl Display for If {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.otherwise {
            Some(otherwise) => write!(f, "If({}, {}, {})", self.expression, self.then, otherwise),
            None => write!(f, "If({}, {})", self.expression, self.then),
        }
    }
}

impl Display for While {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "While({}, {})", self.expression, self.block)
    }
}
