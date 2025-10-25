use crate::ast::{
    arithmetic::{Addition, Division, Multiplication, Subtraction},
    boolean::{Greater, GreaterEqual, Less, LessEqual, LogicalAnd, LogicalNot, LogicalOr},
    rvalues::Integer,
};

pub mod llvmcompiler;

pub trait NodeCompiler {
    fn compile_sum(&self, node: &Addition) -> String;
    fn compile_subtraction(&self, node: &Subtraction) -> String;
    fn compile_multiplication(&self, node: &Multiplication) -> String;
    fn compile_division(&self, node: &Division) -> String;

    fn compile_logical_not(&self, node: &LogicalNot) -> String;
    fn compile_logical_or(&self, node: &LogicalOr) -> String;
    fn compile_logical_and(&self, node: &LogicalAnd) -> String;
    fn compile_less(&self, node: &Less) -> String;
    fn compile_greater(&self, node: &Greater) -> String;
    fn compile_less_equal(&self, node: &LessEqual) -> String;
    fn compile_greater_equal(&self, node: &GreaterEqual) -> String;

    fn compile_assignment(&self, node: &crate::ast::lvalues::Assignment) -> String;
    fn compile_identifier(&self, node: &crate::ast::lvalues::Identifier) -> String;

    fn compile_int_lit(&self, node: &Integer) -> String;
}
