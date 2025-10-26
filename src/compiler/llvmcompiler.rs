use inkwell::{values::InstructionOpcode, builder::Builder, context::Context, module::Module, values::IntValue, basic_block::BasicBlock};
use anyhow::Result;
use crate::{
    ast::{arithmetic::*, rvalues::*, lvalues::*, boolean::*},
    compiler::NodeCompiler,
};

pub struct LlvmCompiler<'a> {
    pub context: &'a Context,
    pub builder: Builder<'a>,
    pub module: Module<'a>,

    binaryLiterals: Vec<IntValue<'a>>,
    blocks: Vec<BasicBlock<'a>>,
}

impl<'a> LlvmCompiler<'a> {
    pub fn new(program_name: &str, context: &'a Context) -> Self {
        let builder = context.create_builder();
        let module = context.create_module(program_name);

        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = module.add_function("main", fn_type, None);
        let main_block = context.append_basic_block(function, "entry");

        LlvmCompiler { context, builder, module, binaryLiterals: Vec::new(), blocks: vec![main_block] }
    }
}

impl NodeCompiler for LlvmCompiler<'_> {
    fn compile(&self) -> Result<String> {
        Ok(self.module.print_to_string().to_string())
    }

    fn compile_sum(&mut self, node: &Addition) -> Result<()> {
        node.left.accept(self)?;
        node.right.accept(self)?;

        let right = self.binaryLiterals.pop().ok_or_else(|| {
            anyhow::anyhow!("Left operand not found for addition")
        })?;
        let left = self.binaryLiterals.pop().ok_or_else(|| {
            anyhow::anyhow!("Right operand not found for addition") 
        })?;

        self.builder.position_at_end(self.blocks.pop().unwrap());
        let _ = self.builder.build_return(Some(&left));

        Ok(())
    }

    fn compile_subtraction(&mut self, node: &Subtraction) -> Result<()> {
        node.left.accept(self)?;
        node.right.accept(self)?;

        Ok(())
    }

    fn compile_multiplication(&mut self, node: &Multiplication) -> Result<()> {
        node.left.accept(self)?;
        node.right.accept(self)?;

        Ok(())
    }

    fn compile_division(&mut self, node: &Division) -> Result<()> {
        node.left.accept(self)?;
        node.right.accept(self)?;

        Ok(())
    }

    fn compile_logical_not(&mut self, node: &LogicalNot) -> Result<()> {
        node.value.accept(self)?;

        Ok(())
    }

    fn compile_logical_or(&mut self, node: &LogicalOr) -> Result<()> {
        node.left.accept(self)?;
        node.right.accept(self)?;

        Ok(())
    }

    fn compile_logical_and(&mut self, node: &LogicalAnd) -> Result<()> {
        node.left.accept(self)?;
        node.right.accept(self)?;

        Ok(())
    }

    fn compile_less(&mut self, node: &Less) -> Result<()> {
        node.left.accept(self)?;
        node.right.accept(self)?;

        Ok(())
    }

    fn compile_greater(&mut self, node: &Greater) -> Result<()> {
        node.left.accept(self)?;
        node.right.accept(self)?;

        Ok(())
    }

    fn compile_less_equal(&mut self, node: &LessEqual) -> Result<()> {
        node.left.accept(self)?;
        node.right.accept(self)?;

        Ok(())
    }

    fn compile_greater_equal(&mut self, node: &GreaterEqual) -> Result<()> {
        node.left.accept(self)?;
        node.right.accept(self)?;

        Ok(())
    }

    fn compile_assignment(&mut self, node: &Assignment) -> Result<()> {
        node.left.accept(self)?;
        node.right.accept(self)?;

        Ok(())
    }

    fn compile_identifier(&mut self, node: &Identifier) -> Result<()> {
        println!("Compiling identifier: {}", node.name);

        Ok(())
    }

    fn compile_int_lit(&mut self, node: &Integer) -> Result<()> {
        let i32_type = self.context.i32_type();
        let value = i32_type.const_int(node.value as u64, false);

        self.binaryLiterals.push(value);

        Ok(())
    }
}
