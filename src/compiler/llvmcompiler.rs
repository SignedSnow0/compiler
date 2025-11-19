use crate::compiler::AstVisitor;
use anyhow::Result;
use crate::ast::*;

pub struct LlvmCompiler<'a> {
    pub context: &'a Context,
    pub builder: Builder<'a>,
    pub module: Module<'a>,

    intermediate_values: Vec<IntValue<'a>>,
    blocks: Vec<BasicBlock<'a>>,
}

impl LlvmCompiler<'_> {
    pub fn new(context: &Context, module_name: &str) -> LlvmCompiler {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        LlvmCompiler {
            context,
            builder,
            module,
            intermediate_values: Vec::new(),
            blocks: Vec::new(),
        }
    }
}

impl AstVisitor for LlvmCompiler {
    fn visit(&mut self, node: &Program) -> Result<()> {
        todo!()
    }

    fn visit_addition(&mut self, node: &Addition) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            self.intermediate_values.pop().unwrap()
        };
        let right = {
            node.right.accept(self)?;
            self.intermediate_values.pop().unwrap()
        };

        let result = self.builder.build_int_add(left, right, "addtmp");
        self.intermediate_values.push(result);

        Ok(())
    }

    fn visit_subtraction(&mut self, node: &Subtraction) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            self.intermediate_values.pop().unwrap()
        };
        let right = {
            node.right.accept(self)?;
            self.intermediate_values.pop().unwrap()
        };

        let result = self.builder.build_int_sub(left, right, "subtmp");
        self.intermediate_values.push(result);

        Ok(())
    }

    fn visit_multiplication(&mut self, node: &Multiplication) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            self.intermediate_values.pop().unwrap()
        };
        let right = {
            node.right.accept(self)?;
            self.intermediate_values.pop().unwrap()
        };

        let result = self.builder.build_int_mul(left, right, "multmp");
        self.intermediate_values.push(result);

        Ok(())
    }

    fn visit_division(&mut self, node: &Division) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            self.intermediate_values.pop().unwrap()
        };
        let right = {
            node.right.accept(self)?;
            self.intermediate_values.pop().unwrap()
        };

        let result = self.builder.build_int_signed_div(left, right, "divtmp");
        self.intermediate_values.push(result);

        Ok(())
    }

    fn visit_integer(&mut self, node: &Integer) -> Result<()> {
        let value = self.context.i32_type().const_int(node.value as u64, false);
        self.intermediate_values.push(value);

        Ok(())
    }

    fn visit_identifier(&mut self, node: &Identifier) -> Result<()> {
        todo!()
    }

    fn visit_declaration(&mut self, node: &Declaration) -> Result<()> {
        todo!()
    }

    fn visit_block(&mut self, node: &Block) -> Result<()> {
        todo!()
    }

    fn visit_function(&mut self, node: &Function) -> Result<()> {
        let ret_type = self.context.i32_type();
        let param_types: Vec<BasicTypeEnum> = node.parameters.iter()
            .map(|_| self.context.i32_type().into())
            .collect();

        let fn_type = ret_type.fn_type(&param_types, false);
        let function = self.module.add_function(&node.name, fn_type, None);
        Ok(())
    }

    fn visit_if(&mut self, node: &If) -> Result<()> {
        todo!()
    }

    fn visit_while(&mut self, node: &While) -> Result<()> {
        todo!()
    }

    fn visit_return(&mut self, node: &Return) -> Result<()> {
        todo!()
    }

    fn visit_function_call(&mut self, node: &FunctionCall) -> Result<()> {
        todo!()
    }

    fn visit_or(&mut self, node: &Or) -> Result<()> {
        todo!()
    }

    fn visit_and(&mut self, node: &And) -> Result<()> {
        todo!()
    }

    fn visit_greater(&mut self, node: &Greater) -> Result<()> {
        todo!()
    }

    fn visit_lesser(&mut self, node: &Lesser) -> Result<()> {
        todo!()
    }

    fn visit_greater_equal(&mut self, node: &GreaterEqual) -> Result<()> {
        todo!()
    }

    fn visit_lesser_equal(&mut self, node: &LesserEqual) -> Result<()> {
        todo!()
    }
}