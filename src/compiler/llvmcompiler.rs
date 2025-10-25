use inkwell::context::Context;

use crate::{
    ast::{arithmetic::Addition, rvalues::Integer},
    compiler::NodeCompiler,
};

pub struct LlvmCompiler {
    pub module: Module,
}

impl LlvmCompiler {
    pub fn new(program_name: &str) -> Self {
        let context = Context::create();
        let module = context.create_module(program_name);

        LlvmCompiler { module }
    }
}

impl NodeCompiler for LlvmCompiler {
    fn compile_sum(&self, node: &Addition) -> String {
        format!("{} + {}", node.left.accept(self), node.right.accept(self))
    }

    fn compile_subtraction(&self, node: &crate::ast::arithmetic::Subtraction) -> String {
        format!("{} - {}", node.left.accept(self), node.right.accept(self))
    }

    fn compile_multiplication(&self, node: &crate::ast::arithmetic::Multiplication) -> String {
        format!("{} * {}", node.left.accept(self), node.right.accept(self))
    }

    fn compile_division(&self, node: &crate::ast::arithmetic::Division) -> String {
        format!("{} / {}", node.left.accept(self), node.right.accept(self))
    }

    fn compile_logical_not(&self, node: &crate::ast::boolean::LogicalNot) -> String {
        format!("!{}", node.value.accept(self))
    }

    fn compile_logical_or(&self, node: &crate::ast::boolean::LogicalOr) -> String {
        format!("{} || {}", node.left.accept(self), node.right.accept(self))
    }

    fn compile_logical_and(&self, node: &crate::ast::boolean::LogicalAnd) -> String {
        format!("{} && {}", node.left.accept(self), node.right.accept(self))
    }

    fn compile_less(&self, node: &crate::ast::boolean::Less) -> String {
        format!("{} < {}", node.left.accept(self), node.right.accept(self))
    }

    fn compile_greater(&self, node: &crate::ast::boolean::Greater) -> String {
        format!("{} > {}", node.left.accept(self), node.right.accept(self))
    }

    fn compile_less_equal(&self, node: &crate::ast::boolean::LessEqual) -> String {
        format!("{} <= {}", node.left.accept(self), node.right.accept(self))
    }

    fn compile_greater_equal(&self, node: &crate::ast::boolean::GreaterEqual) -> String {
        format!("{} >= {}", node.left.accept(self), node.right.accept(self))
    }

    fn compile_assignment(&self, node: &crate::ast::lvalues::Assignment) -> String {
        format!("{} = {}", node.left.accept(self), node.right.accept(self))
    }

    fn compile_identifier(&self, node: &crate::ast::lvalues::Identifier) -> String {
        format!("{}", node.name)
    }

    fn compile_int_lit(&self, node: &Integer) -> String {
        format!("{}", node.value)
    }
}
