use crate::ast::{
    Addition, And, Assignment, AstNode, Block, Declaration, Division, Equality, Function,
    FunctionCall, Greater, GreaterEqual, Identifier, If, Inequality, Integer, Lesser, LesserEqual,
    Multiplication, Or, Program, Return, StructTypedef, Subtraction, Type, While,
};
use anyhow::Result;

pub mod llvmcompiler;
mod type_checker;
pub mod type_converter;

pub trait AstVisitor {
    fn visit(&mut self, node: &Program) -> Result<()>;

    fn visit_addition(&mut self, node: &Addition) -> Result<()>;
    fn visit_subtraction(&mut self, node: &Subtraction) -> Result<()>;
    fn visit_multiplication(&mut self, node: &Multiplication) -> Result<()>;
    fn visit_division(&mut self, node: &Division) -> Result<()>;
    fn visit_integer(&mut self, node: &Integer) -> Result<()>;
    fn visit_identifier(&mut self, node: &Identifier) -> Result<()>;
    fn visit_declaration(&mut self, node: &Declaration) -> Result<()>;
    fn visit_block(&mut self, node: &Block) -> Result<()>;
    fn visit_function(&mut self, node: &Function) -> Result<()>;
    fn visit_if(&mut self, node: &If) -> Result<()>;
    fn visit_while(&mut self, node: &While) -> Result<()>;
    fn visit_return(&mut self, node: &Return) -> Result<()>;
    fn visit_function_call(&mut self, node: &FunctionCall) -> Result<()>;
    fn visit_or(&mut self, node: &Or) -> Result<()>;
    fn visit_and(&mut self, node: &And) -> Result<()>;
    fn visit_equality(&mut self, node: &Equality) -> Result<()>;
    fn visit_inequality(&mut self, node: &Inequality) -> Result<()>;
    fn visit_greater(&mut self, node: &Greater) -> Result<()>;
    fn visit_lesser(&mut self, node: &Lesser) -> Result<()>;
    fn visit_greater_equal(&mut self, node: &GreaterEqual) -> Result<()>;
    fn visit_lesser_equal(&mut self, node: &LesserEqual) -> Result<()>;
    fn visit_assignment(&mut self, node: &Assignment) -> Result<()>;
    fn visit_typedef(&mut self, node: &StructTypedef) -> Result<()>;
}

pub struct AstWriter;

impl AstVisitor for AstWriter {
    fn visit(&mut self, node: &Program) -> Result<()> {
        print!("Program(");
        for (i, child) in node.nodes.iter().enumerate() {
            child.accept(self)?;
            if i < node.nodes.len() - 1 {
                print!(", ");
            }
        }

        print!(")");
        Ok(())
    }

    fn visit_addition(&mut self, node: &Addition) -> Result<()> {
        print!("Addition(");
        node.left.accept(self)?;
        print!(", ");
        node.right.accept(self)?;
        print!(")");
        Ok(())
    }

    fn visit_subtraction(&mut self, node: &Subtraction) -> Result<()> {
        print!("Subtraction(");
        node.left.accept(self)?;
        print!(", ");
        node.right.accept(self)?;
        print!(")");
        Ok(())
    }

    fn visit_multiplication(&mut self, node: &Multiplication) -> Result<()> {
        print!("Multiplication(");
        node.left.accept(self)?;
        print!(", ");
        node.right.accept(self)?;
        print!(")");
        Ok(())
    }

    fn visit_division(&mut self, node: &Division) -> Result<()> {
        print!("Division(");
        node.left.accept(self)?;
        print!(", ");
        node.right.accept(self)?;
        print!(")");
        Ok(())
    }

    fn visit_integer(&mut self, node: &Integer) -> Result<()> {
        print!("Integer({})", node.value);
        Ok(())
    }

    fn visit_identifier(&mut self, node: &Identifier) -> Result<()> {
        print!("Identifier({})", node.name);
        Ok(())
    }

    fn visit_declaration(&mut self, node: &Declaration) -> Result<()> {
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

        print!("Declaration({}, {}, ", node.name, type_name);
        match &node.value {
            None => print!("None"),
            Some(_) => node.value.as_ref().unwrap().accept(self)?,
        }
        print!(")");
        Ok(())
    }

    fn visit_block(&mut self, node: &Block) -> Result<()> {
        print!("Block(");
        for (i, child) in node.nodes.iter().enumerate() {
            child.accept(self)?;
            if i < node.nodes.len() - 1 {
                print!(", ");
            }
        }
        print!(")");
        Ok(())
    }

    fn visit_function(&mut self, node: &Function) -> Result<()> {
        print!("Function({}, [", node.name);
        for (i, (name, param)) in node.parameters.iter().enumerate() {
            let type_name = match param {
                Type::Integer32 => "i32",
                Type::Boolean8 => "b8",
                Type::Custom(custom_name) => custom_name.as_str(),
            };
            print!("({}, {})", name, type_name);
            if i < node.parameters.len() - 1 {
                print!(", ");
            }
        }
        if let Some(return_type) = &node.return_type {
            let return_type = match &return_type {
                Type::Integer32 => "i32",
                Type::Boolean8 => "b8",
                Type::Custom(custom_name) => custom_name.as_str(),
            };
            print!("-> {}], ", return_type);
        } else {
            print!("], None, ");
        }

        node.body.accept(self)?;
        print!(")");
        Ok(())
    }

    fn visit_if(&mut self, node: &If) -> Result<()> {
        print!("If(");
        node.condition.accept(self)?;
        print!(", ");
        node.then_block.accept(self)?;
        if let Some(else_block) = &node.else_block {
            print!(", ");
            else_block.accept(self)?;
        }
        print!(")");
        Ok(())
    }

    fn visit_while(&mut self, node: &While) -> Result<()> {
        print!("While(");
        node.condition.accept(self)?;
        print!(", ");
        node.block.accept(self)?;
        print!(")");
        Ok(())
    }

    fn visit_return(&mut self, node: &Return) -> Result<()> {
        print!("Return(");
        node.value.accept(self)?;
        print!(")");
        Ok(())
    }

    fn visit_function_call(&mut self, node: &FunctionCall) -> Result<()> {
        print!("FunctionCall({}, [", node.name);
        for (i, arg) in node.arguments.iter().enumerate() {
            arg.accept(self)?;
            if i < node.arguments.len() - 1 {
                print!(", ");
            }
        }
        print!("])");
        Ok(())
    }

    fn visit_or(&mut self, node: &Or) -> Result<()> {
        print!("Or(");
        node.left.accept(self)?;
        print!(", ");
        node.right.accept(self)?;
        print!(")");
        Ok(())
    }

    fn visit_and(&mut self, node: &And) -> Result<()> {
        print!("And(");
        node.left.accept(self)?;
        print!(", ");
        node.right.accept(self)?;
        print!(")");
        Ok(())
    }

    fn visit_equality(&mut self, node: &Equality) -> Result<()> {
        print!("Equality(");
        node.left.accept(self)?;
        print!(", ");
        node.right.accept(self)?;
        print!(")");
        Ok(())
    }

    fn visit_inequality(&mut self, node: &Inequality) -> Result<()> {
        print!("Inequality(");
        node.left.accept(self)?;
        print!(", ");
        node.right.accept(self)?;
        print!(")");
        Ok(())
    }

    fn visit_greater(&mut self, node: &Greater) -> Result<()> {
        print!("Greater(");
        node.left.accept(self)?;
        print!(", ");
        node.right.accept(self)?;
        print!(")");
        Ok(())
    }

    fn visit_lesser(&mut self, node: &Lesser) -> Result<()> {
        print!("Lesser(");
        node.left.accept(self)?;
        print!(", ");
        node.right.accept(self)?;
        print!(")");
        Ok(())
    }

    fn visit_greater_equal(&mut self, node: &GreaterEqual) -> Result<()> {
        print!("GreaterEqual(");
        node.left.accept(self)?;
        print!(", ");
        node.right.accept(self)?;
        print!(")");
        Ok(())
    }

    fn visit_lesser_equal(&mut self, node: &LesserEqual) -> Result<()> {
        print!("LesserEqual(");
        node.left.accept(self)?;
        print!(", ");
        node.right.accept(self)?;
        print!(")");
        Ok(())
    }

    fn visit_assignment(&mut self, node: &Assignment) -> Result<()> {
        print!("Assignment({}, ", node.target);
        node.accept(self)?;
        print!(")");
        Ok(())
    }

    fn visit_typedef(&mut self, node: &StructTypedef) -> Result<()> {
        print!("Typedef({}, [", node.name);
        for (i, (name, f_type)) in node.fields.iter().enumerate() {
            let f_type = match f_type {
                Type::Integer32 => "i32",
                Type::Boolean8 => "b8",
                Type::Custom(custom_name) => custom_name.as_str(),
            };
            print!("({}, {})", name, f_type);

            if i < node.fields.len() - 1 {
                print!(", ");
            }
        }
        print!("])");
        Ok(())
    }
}
