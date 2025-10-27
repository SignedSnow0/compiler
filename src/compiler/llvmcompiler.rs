use crate::{
    ast::{arithmetic::*, boolean::*, lvalues::*, rvalues::*},
    compiler::NodeCompiler,
};
use anyhow::Result;
use inkwell::{
    basic_block::BasicBlock, builder::Builder, context::Context, module::Module, values::IntValue,
};

pub struct LlvmCompiler<'a> {
    pub context: &'a Context,
    pub builder: Builder<'a>,
    pub module: Module<'a>,

    intermediate_values: Vec<IntValue<'a>>,
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

        LlvmCompiler {
            context,
            builder,
            module,
            intermediate_values: Vec::new(),
            blocks: vec![main_block],
        }
    }
}

impl NodeCompiler for LlvmCompiler<'_> {
    fn compile(&mut self) -> Result<String> {
        self.builder.position_at_end(self.blocks.pop().unwrap());
        let _ = self
            .builder
            .build_return(Some(&self.intermediate_values.pop().unwrap()));

        Ok(self.module.print_to_string().to_string())
    }

    fn compile_sum(&mut self, node: &Addition) -> Result<()> {
        node.left.accept(self)?;
        node.right.accept(self)?;

        let right = self
            .intermediate_values
            .pop()
            .ok_or_else(|| anyhow::anyhow!("Left operand not found for addition"))?;
        let left = self
            .intermediate_values
            .pop()
            .ok_or_else(|| anyhow::anyhow!("Right operand not found for addition"))?;

        self.builder.position_at_end(*self.blocks.last().unwrap());
        self.intermediate_values
            .push(self.builder.build_int_add(left, right, "add").unwrap());

        Ok(())
    }

    fn compile_subtraction(&mut self, node: &Subtraction) -> Result<()> {
        node.left.accept(self)?;
        node.right.accept(self)?;

        let right = self
            .intermediate_values
            .pop()
            .ok_or_else(|| anyhow::anyhow!("Left operand not found for subtraction"))?;
        let left = self
            .intermediate_values
            .pop()
            .ok_or_else(|| anyhow::anyhow!("Right operand not found for subtraction"))?;

        self.builder.position_at_end(*self.blocks.last().unwrap());
        self.intermediate_values
            .push(self.builder.build_int_sub(left, right, "sub").unwrap());

        Ok(())
    }

    fn compile_multiplication(&mut self, node: &Multiplication) -> Result<()> {
        node.left.accept(self)?;
        node.right.accept(self)?;

        let right = self
            .intermediate_values
            .pop()
            .ok_or_else(|| anyhow::anyhow!("Left operand not found for multiplication"))?;
        let left = self
            .intermediate_values
            .pop()
            .ok_or_else(|| anyhow::anyhow!("Right operand not found for multiplication"))?;

        self.builder.position_at_end(*self.blocks.last().unwrap());
        self.intermediate_values
            .push(self.builder.build_int_mul(left, right, "mul").unwrap());

        Ok(())
    }

    fn compile_division(&mut self, node: &Division) -> Result<()> {
        node.left.accept(self)?;
        node.right.accept(self)?;

        let right = self
            .intermediate_values
            .pop()
            .ok_or_else(|| anyhow::anyhow!("Left operand not found for division"))?;
        let left = self
            .intermediate_values
            .pop()
            .ok_or_else(|| anyhow::anyhow!("Right operand not found for division"))?;

        self.builder.position_at_end(*self.blocks.last().unwrap());
        self.intermediate_values.push(
            self.builder
                .build_int_signed_div(left, right, "div")
                .unwrap(),
        );

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
        Ok(())
    }

    fn compile_int_lit(&mut self, node: &Integer) -> Result<()> {
        let i32_type = self.context.i32_type();
        let value = i32_type.const_int(node.value.try_into().unwrap(), false);
        self.intermediate_values.push(value);

        Ok(())
    }
}
