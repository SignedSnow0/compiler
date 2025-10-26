use crate::ast::{
    arithmetic::{Addition, Division, Multiplication, Subtraction},
    boolean::{Greater, GreaterEqual, Less, LessEqual, LogicalAnd, LogicalNot, LogicalOr},
    rvalues::Integer,
    lvalues::{Assignment, Identifier},
};
use anyhow::Result;

pub mod llvmcompiler;

pub trait NodeCompiler {
    fn compile(&self) -> Result<String>;

    fn compile_sum(&mut self, node: &Addition) -> Result<()>;
    fn compile_subtraction(&mut self, node: &Subtraction) -> Result<()>;
    fn compile_multiplication(&mut self, node: &Multiplication) -> Result<()>;
    fn compile_division(&mut self, node: &Division) -> Result<()>;

    fn compile_logical_not(&mut self, node: &LogicalNot) -> Result<()>;
    fn compile_logical_or(&mut self, node: &LogicalOr) -> Result<()>;
    fn compile_logical_and(&mut self, node: &LogicalAnd) -> Result<()>;
    fn compile_less(&mut self, node: &Less) -> Result<()>;
    fn compile_greater(&mut self, node: &Greater) -> Result<()>;
    fn compile_less_equal(&mut self, node: &LessEqual) -> Result<()>;
    fn compile_greater_equal(&mut self, node: &GreaterEqual) -> Result<()>;

    fn compile_assignment(&mut self, node: &Assignment) -> Result<()>;
    fn compile_identifier(&mut self, node: &Identifier) -> Result<()>;

    fn compile_int_lit(&mut self, node: &Integer) -> Result<()>;
}
