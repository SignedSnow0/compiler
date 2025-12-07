use crate::ast::{
    Addition, And, Assignment, AstNode, Block, Declaration, Division, Equality, Function,
    FunctionCall, Greater, GreaterEqual, Identifier, If, Inequality, Integer, Lesser, LesserEqual,
    Multiplication, Or, Program, Return, StructTypedef, Subtraction, Type, While,
};
use anyhow::Result;
use std::fmt::Write;

pub mod llvmcompiler;
mod type_checker;
pub mod type_converter;

pub trait AstVisitor {
    fn visit(&mut self, node: &mut Program) -> Result<()>;

    fn visit_addition(&mut self, node: &mut Addition) -> Result<()>;
    fn visit_subtraction(&mut self, node: &mut Subtraction) -> Result<()>;
    fn visit_multiplication(&mut self, node: &mut Multiplication) -> Result<()>;
    fn visit_division(&mut self, node: &mut Division) -> Result<()>;
    fn visit_integer(&mut self, node: &mut Integer) -> Result<()>;
    fn visit_identifier(&mut self, node: &mut Identifier) -> Result<()>;
    fn visit_declaration(&mut self, node: &mut Declaration) -> Result<()>;
    fn visit_block(&mut self, node: &mut Block) -> Result<()>;
    fn visit_function(&mut self, node: &mut Function) -> Result<()>;
    fn visit_if(&mut self, node: &mut If) -> Result<()>;
    fn visit_while(&mut self, node: &mut While) -> Result<()>;
    fn visit_return(&mut self, node: &mut Return) -> Result<()>;
    fn visit_function_call(&mut self, node: &mut FunctionCall) -> Result<()>;
    fn visit_or(&mut self, node: &mut Or) -> Result<()>;
    fn visit_and(&mut self, node: &mut And) -> Result<()>;
    fn visit_equality(&mut self, node: &mut Equality) -> Result<()>;
    fn visit_inequality(&mut self, node: &mut Inequality) -> Result<()>;
    fn visit_greater(&mut self, node: &mut Greater) -> Result<()>;
    fn visit_lesser(&mut self, node: &mut Lesser) -> Result<()>;
    fn visit_greater_equal(&mut self, node: &mut GreaterEqual) -> Result<()>;
    fn visit_lesser_equal(&mut self, node: &mut LesserEqual) -> Result<()>;
    fn visit_assignment(&mut self, node: &mut Assignment) -> Result<()>;
    fn visit_typedef(&mut self, node: &mut StructTypedef) -> Result<()>;
}

pub struct AstWriter {
    value: String,
}

impl AstWriter {
    pub fn new() -> Self {
        Self {
            value: String::new(),
        }
    }

    fn write(&mut self, value: &str) -> Result<()> {
        Ok(write!(&mut self.value, "{}", value)?)
    }

    pub fn get_string(&self) -> String {
        self.value.clone()
    }
}

impl AstVisitor for AstWriter {
    fn visit(&mut self, node: &mut Program) -> Result<()> {
        self.write("Program(")?;
        let nodes_len = node.nodes.len();
        for (i, child) in node.nodes.iter_mut().enumerate() {
            child.accept(self)?;
            if i < nodes_len - 1 {
                self.write(", ")?;
            }
        }

        self.write(")")?;
        Ok(())
    }

    fn visit_addition(&mut self, node: &mut Addition) -> Result<()> {
        self.write("Addition(")?;
        node.left.accept(self)?;
        self.write(", ")?;
        node.right.accept(self)?;
        self.write(")")?;
        Ok(())
    }

    fn visit_subtraction(&mut self, node: &mut Subtraction) -> Result<()> {
        self.write("Subtraction(")?;
        node.left.accept(self)?;
        self.write(", ")?;
        node.right.accept(self)?;
        self.write(")")?;
        Ok(())
    }

    fn visit_multiplication(&mut self, node: &mut Multiplication) -> Result<()> {
        self.write("Multiplication(")?;
        node.left.accept(self)?;
        self.write(", ")?;
        node.right.accept(self)?;
        self.write(")")?;
        Ok(())
    }

    fn visit_division(&mut self, node: &mut Division) -> Result<()> {
        self.write("Division(")?;
        node.left.accept(self)?;
        self.write(", ")?;
        node.right.accept(self)?;
        self.write(")")?;
        Ok(())
    }

    fn visit_integer(&mut self, node: &mut Integer) -> Result<()> {
        self.write("Integer(")?;
        self.write(&node.value.to_string())?;
        self.write(")")?;
        Ok(())
    }

    fn visit_identifier(&mut self, node: &mut Identifier) -> Result<()> {
        self.write("Identifier(")?;
        self.write(&node.name)?;
        self.write(")")?;
        Ok(())
    }

    fn visit_declaration(&mut self, node: &mut Declaration) -> Result<()> {
        let type_name = {
            if let Some(t) = &node.d_type {
                match t {
                    Type::Integer32 => "i32",
                    Type::Boolean8 => "b8",
                    Type::Custom(name) => name.as_str(),
                }
            } else {
                "None"
            }
        };

        self.write("Declaration(")?;
        self.write(&node.name)?;
        self.write(", ")?;
        self.write(type_name)?;

        print!("Declaration({}, {}, ", node.name, type_name);
        match &mut node.value {
            None => {}
            Some(_) => {
                self.write(", ")?;
                node.value.as_mut().unwrap().accept(self)?;
            }
        }
        print!(")");
        Ok(())
    }

    fn visit_block(&mut self, node: &mut Block) -> Result<()> {
        self.write("Block(")?;
        let nodes_len = node.nodes.len();
        for (i, child) in node.nodes.iter_mut().enumerate() {
            child.as_mut().accept(self)?;
            if i < nodes_len - 1 {
                self.write(", ")?;
            }
        }
        self.write(")")?;
        Ok(())
    }

    fn visit_function(&mut self, node: &mut Function) -> Result<()> {
        self.write("Function(")?;
        self.write(&node.name)?;
        self.write(", [")?;
        for (i, (name, param)) in node.parameters.iter().enumerate() {
            let type_name = match param {
                Type::Integer32 => "i32",
                Type::Boolean8 => "b8",
                Type::Custom(custom_name) => custom_name.as_str(),
            };
            self.write("(")?;
            self.write(&name)?;
            self.write(", ")?;
            self.write(type_name)?;
            self.write(")")?;
            if i < node.parameters.len() - 1 {
                self.write(" ")?;
            }
        }
        if let Some(return_type) = &node.return_type {
            let return_type = match &return_type {
                Type::Integer32 => "i32",
                Type::Boolean8 => "b8",
                Type::Custom(custom_name) => custom_name.as_str(),
            };

            self.write("-> ")?;
            self.write(return_type)?;
            self.write("], ")?;
        } else {
            self.write("], ")?;
        }

        node.body.accept(self)?;
        self.write(")")?;
        Ok(())
    }

    fn visit_if(&mut self, node: &mut If) -> Result<()> {
        self.write("If(")?;
        node.condition.accept(self)?;
        self.write(", ")?;
        node.then_block.accept(self)?;
        if let Some(else_block) = &mut node.else_block {
            self.write(", ")?;
            else_block.accept(self)?;
        }
        self.write(")")?;
        Ok(())
    }

    fn visit_while(&mut self, node: &mut While) -> Result<()> {
        self.write("While(")?;
        node.condition.accept(self)?;
        self.write(", ")?;
        node.block.accept(self)?;
        self.write(")")?;
        Ok(())
    }

    fn visit_return(&mut self, node: &mut Return) -> Result<()> {
        self.write("Return(")?;
        node.value.accept(self)?;
        self.write(")")?;
        Ok(())
    }

    fn visit_function_call(&mut self, node: &mut FunctionCall) -> Result<()> {
        self.write("FunctionCall(")?;
        self.write(&node.name)?;
        self.write(", [")?;
        let nodes_len = node.arguments.len();
        for (i, arg) in node.arguments.iter_mut().enumerate() {
            arg.accept(self)?;
            if i < nodes_len - 1 {
                self.write(", ")?;
            }
        }
        self.write("])")?;
        Ok(())
    }

    fn visit_or(&mut self, node: &mut Or) -> Result<()> {
        self.write("Or(")?;
        node.left.accept(self)?;
        self.write(", ")?;
        node.right.accept(self)?;
        self.write(")")?;
        Ok(())
    }

    fn visit_and(&mut self, node: &mut And) -> Result<()> {
        self.write("And(")?;
        node.left.accept(self)?;
        self.write(", ")?;
        node.right.accept(self)?;
        self.write(")")?;
        Ok(())
    }

    fn visit_equality(&mut self, node: &mut Equality) -> Result<()> {
        self.write("Equality(")?;
        node.left.accept(self)?;
        self.write(", ")?;
        node.right.accept(self)?;
        self.write(")")?;
        Ok(())
    }

    fn visit_inequality(&mut self, node: &mut Inequality) -> Result<()> {
        self.write("Inequality(")?;
        node.left.accept(self)?;
        self.write(", ")?;
        node.right.accept(self)?;
        self.write(")")?;
        Ok(())
    }

    fn visit_greater(&mut self, node: &mut Greater) -> Result<()> {
        self.write("Greater(")?;
        node.left.accept(self)?;
        self.write(", ")?;
        node.right.accept(self)?;
        self.write(")")?;
        Ok(())
    }

    fn visit_lesser(&mut self, node: &mut Lesser) -> Result<()> {
        self.write("Lesser(")?;
        node.left.accept(self)?;
        self.write(", ")?;
        node.right.accept(self)?;
        self.write(")")?;
        Ok(())
    }

    fn visit_greater_equal(&mut self, node: &mut GreaterEqual) -> Result<()> {
        self.write("GreaterEqual(")?;
        node.left.accept(self)?;
        self.write(", ")?;
        node.right.accept(self)?;
        self.write(")")?;
        Ok(())
    }

    fn visit_lesser_equal(&mut self, node: &mut LesserEqual) -> Result<()> {
        self.write("LesserEqual(")?;
        node.left.accept(self)?;
        self.write(", ")?;
        node.right.accept(self)?;
        self.write(")")?;
        Ok(())
    }

    fn visit_assignment(&mut self, node: &mut Assignment) -> Result<()> {
        self.write("Assignment(")?;
        self.write(&node.target)?;
        self.write(", ")?;
        node.accept(self)?;
        self.write(")")?;
        Ok(())
    }

    fn visit_typedef(&mut self, node: &mut StructTypedef) -> Result<()> {
        self.write("Typedef(")?;
        self.write(&node.name)?;
        self.write(", [")?;
        for (i, (name, f_type)) in node.fields.iter().enumerate() {
            let f_type = match f_type {
                Type::Integer32 => "i32",
                Type::Boolean8 => "b8",
                Type::Custom(custom_name) => custom_name.as_str(),
            };
            self.write("(")?;
            self.write(&name)?;
            self.write(", ")?;
            self.write(f_type)?;
            self.write(")")?;

            if i < node.fields.len() - 1 {
                self.write(", ")?;
            }
        }
        self.write("])")?;
        Ok(())
    }
}
