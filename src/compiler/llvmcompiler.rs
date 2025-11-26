use crate::ast::*;
use crate::compiler::AstVisitor;
use anyhow::{Result, anyhow};
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    types::{
        BasicMetadataTypeEnum,
        BasicTypeEnum::{IntType, PointerType},
    },
    values::{BasicValueEnum, FunctionValue, PointerValue},
};
use std::collections::HashMap;

struct FunctionContext<'a> {
    function: FunctionValue<'a>,
    variables: HashMap<String, PointerValue<'a>>, //scoped context
}

pub struct LlvmCompiler<'a> {
    context: &'a Context,
    builder: Builder<'a>,
    module: Module<'a>,

    intermediate_values: Vec<BasicValueEnum<'a>>,
    current_function: Option<FunctionContext<'a>>,
    global_variables: HashMap<String, PointerValue<'a>>,
    defined_functions: HashMap<String, FunctionValue<'a>>,
}

impl LlvmCompiler<'_> {
    pub fn new<'a>(context: &'a Context, module_name: &str) -> LlvmCompiler<'a> {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        LlvmCompiler {
            context,
            builder,
            module,
            intermediate_values: Vec::new(),
            current_function: None,
            global_variables: HashMap::new(),
            defined_functions: HashMap::new(),
        }
    }

    pub fn compile(&mut self) -> Result<String> {
        Ok(self.module.print_to_string().to_string())
    }
}

impl AstVisitor for LlvmCompiler<'_> {
    fn visit(&mut self, node: &Program) -> Result<()> {
        for node in &node.nodes {
            node.accept(self)?;
        }

        Ok(())
    }

    fn visit_addition(&mut self, node: &Addition) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };
        let right = {
            node.right.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };

        let result = self.builder.build_int_add(left, right, "")?;
        self.intermediate_values.push(result.into());

        Ok(())
    }

    fn visit_subtraction(&mut self, node: &Subtraction) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };
        let right = {
            node.right.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };

        let result = self.builder.build_int_sub(left, right, "")?;
        self.intermediate_values.push(result.into());

        Ok(())
    }

    fn visit_multiplication(&mut self, node: &Multiplication) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };
        let right = {
            node.right.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };

        let result = self.builder.build_int_mul(left, right, "")?;
        self.intermediate_values.push(result.into());

        Ok(())
    }

    fn visit_division(&mut self, node: &Division) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };
        let right = {
            node.right.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };

        let result = self.builder.build_int_signed_div(left, right, "")?;
        self.intermediate_values.push(result.into());

        Ok(())
    }

    fn visit_integer(&mut self, node: &Integer) -> Result<()> {
        let value = self.context.i32_type().const_int(node.value as u64, false);
        self.intermediate_values.push(value.into());

        Ok(())
    }

    fn visit_identifier(&mut self, node: &Identifier) -> Result<()> {
        let function_context = match &self.current_function {
            Some(f) => f,
            None => {
                return Err(anyhow!(
                    "Cannot use a variable as a lvalue outside of a function"
                ));
            }
        };

        let variable = match function_context.variables.get(&node.name) {
            Some(var) => var,
            None => {
                if let Some(var) = self.global_variables.get(&node.name) {
                    var
                } else {
                    return Err(anyhow!("Identifier not defined"));
                }
            }
        };

        let value = self
            .builder
            .build_load(self.context.i32_type(), *variable, "")?;
        self.intermediate_values.push(value);

        Ok(())
    }

    fn visit_declaration(&mut self, node: &Declaration) -> Result<()> {
        match &node.value {
            None => {
                return Err(anyhow!(
                    "Variable declaration must include an initial value"
                ));
            }
            Some(value) => value.accept(self)?,
        };

        let value = self.intermediate_values.pop().unwrap();
        let var = self
            .builder
            .build_alloca(self.context.i32_type(), &node.name)?;

        let _ = self.builder.build_store(var, value)?;

        match &mut self.current_function {
            Some(f) => f.variables.insert(node.name.clone(), var),
            None => self.global_variables.insert(node.name.clone(), var),
        };

        Ok(())
    }

    fn visit_block(&mut self, node: &Block) -> Result<()> {
        for statement in &node.nodes {
            statement.accept(self)?;
        }

        Ok(())
    }

    fn visit_function(&mut self, node: &Function) -> Result<()> {
        let ret_type = self.context.i32_type();
        let param_types: Vec<BasicMetadataTypeEnum<'_>> = node
            .parameters
            .iter()
            .map(|_| self.context.i32_type().into())
            .collect();

        let fn_type = ret_type.fn_type(&param_types, false);
        let function = self.module.add_function(&node.name, fn_type, None);

        self.defined_functions.insert(node.name.clone(), function);

        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        let variables: Vec<_> = param_types
            .iter()
            .map(|p| self.builder.build_alloca(p.into_int_type(), "").unwrap())
            .collect();
        variables
            .iter()
            .zip(function.get_params())
            .for_each(|(v, p)| {
                let _ = self.builder.build_store(*v, p);
            });

        let variables: HashMap<String, _> =
            node.parameters.keys().cloned().zip(variables).collect();

        self.current_function = Some(FunctionContext {
            function,
            variables,
        });

        node.body.accept(self)?;

        self.current_function = None;

        Ok(())
    }

    fn visit_if(&mut self, node: &If) -> Result<()> {
        let function = match &self.current_function {
            Some(f) => f.function,
            None => return Err(anyhow!("Cannot call if block outside of a function")),
        };

        let expression = {
            node.condition.accept(self)?;
            self.intermediate_values.pop().unwrap().into_int_value()
        };
        let then_bb = self.context.append_basic_block(function, "then");
        let else_bb = self.context.append_basic_block(function, "else");

        let _ = self
            .builder
            .build_conditional_branch(expression, then_bb, else_bb)?;

        self.builder.position_at_end(then_bb);
        node.then_block.accept(self)?;

        self.builder.position_at_end(else_bb);
        if let Some(else_block) = &node.else_block {
            else_block.accept(self)?;
        }

        //let _ = self.context.append_basic_block(function, "");

        Ok(())
    }

    fn visit_while(&mut self, node: &While) -> Result<()> {
        let function = match &self.current_function {
            Some(f) => f.function,
            None => return Err(anyhow!("Cannot call while block outside of a function")),
        };

        let expression = {
            node.condition.accept(self)?;
            self.intermediate_values.pop().unwrap().into_int_value()
        };

        let while_bb = self.context.append_basic_block(function, "while");
        let then_bb = self.context.append_basic_block(function, "then");

        let _instruction = self
            .builder
            .build_conditional_branch(expression, while_bb, then_bb)?;

        self.builder.position_at_end(while_bb);
        node.block.accept(self)?;

        let expression = {
            node.condition.accept(self)?;
            self.intermediate_values.pop().unwrap().into_int_value()
        };

        let _instruction = self
            .builder
            .build_conditional_branch(expression, while_bb, then_bb)?;

        self.builder.position_at_end(then_bb);

        Ok(())
    }

    fn visit_return(&mut self, node: &Return) -> Result<()> {
        if self.current_function.is_none() {
            return Err(anyhow!("Cannot use return outside of a function"));
        }

        let value = {
            node.value.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };
        self.builder.build_return(Some(&value))?;

        Ok(())
    }

    fn visit_function_call(&mut self, node: &FunctionCall) -> Result<()> {
        let mut arguments = Vec::new();
        for arg in &node.arguments {
            arg.accept(self)?;
            arguments.push(self.intermediate_values.pop().unwrap().into());
        }

        match self.defined_functions.get(&node.name) {
            Some(func) => match func.get_type().get_return_type() {
                Some(_) => {
                    let ret = self
                        .builder
                        .build_call(*func, &arguments, "")?
                        .try_as_basic_value()
                        .unwrap_basic();
                    self.intermediate_values.push(ret);
                }
                None => {
                    self.builder.build_call(*func, &arguments, "")?;
                }
            },
            None => return Err(anyhow::anyhow!("Undefined function called")),
        };

        Ok(())
    }

    fn visit_or(&mut self, node: &Or) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            self.intermediate_values.pop().unwrap().into_int_value()
        };
        let right = {
            node.right.accept(self)?;
            self.intermediate_values.pop().unwrap().into_int_value()
        };

        let result = self.builder.build_or(left, right, "")?;
        self.intermediate_values.push(result.into());

        Ok(())
    }

    fn visit_and(&mut self, node: &And) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            self.intermediate_values.pop().unwrap().into_int_value()
        };
        let right = {
            node.right.accept(self)?;
            self.intermediate_values.pop().unwrap().into_int_value()
        };

        let result = self.builder.build_and(left, right, "")?;
        self.intermediate_values.push(result.into());

        Ok(())
    }

    fn visit_equality(&mut self, node: &Equality) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };
        let right = {
            node.right.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };

        let result = self
            .builder
            .build_int_compare(inkwell::IntPredicate::EQ, left, right, "")?;
        self.intermediate_values.push(result.into());

        Ok(())
    }

    fn visit_inequality(&mut self, node: &Inequality) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };
        let right = {
            node.right.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };

        let result = self
            .builder
            .build_int_compare(inkwell::IntPredicate::NE, left, right, "")?;
        self.intermediate_values.push(result.into());

        Ok(())
    }

    fn visit_greater(&mut self, node: &Greater) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };
        let right = {
            node.right.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };

        let result = self
            .builder
            .build_int_compare(inkwell::IntPredicate::SGT, left, right, "")?;
        self.intermediate_values.push(result.into());

        Ok(())
    }

    fn visit_lesser(&mut self, node: &Lesser) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };
        let right = {
            node.right.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };

        let result = self
            .builder
            .build_int_compare(inkwell::IntPredicate::SLT, left, right, "")?;
        self.intermediate_values.push(result.into());

        Ok(())
    }

    fn visit_greater_equal(&mut self, node: &GreaterEqual) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };
        let right = {
            node.right.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };

        let result = self
            .builder
            .build_int_compare(inkwell::IntPredicate::SGE, left, right, "")?;
        self.intermediate_values.push(result.into());

        Ok(())
    }

    fn visit_lesser_equal(&mut self, node: &LesserEqual) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };
        let right = {
            node.right.accept(self)?;
            match self.intermediate_values.last().unwrap().get_type() {
                IntType(_) => self.intermediate_values.pop().unwrap().into_int_value(),
                PointerType(_) => {
                    let value = self.intermediate_values.pop().unwrap().into_pointer_value();
                    self.builder
                        .build_load(self.context.i32_type(), value, "")?
                        .into_int_value()
                }
                _ => return Err(anyhow!("Unknown type conversion")),
            }
        };

        let result = self
            .builder
            .build_int_compare(inkwell::IntPredicate::SLE, left, right, "")?;
        self.intermediate_values.push(result.into());

        Ok(())
    }

    fn visit_assignment(&mut self, node: &Assignment) -> Result<()> {
        let value = {
            node.value.accept(self)?;
            self.intermediate_values.pop().unwrap()
        };

        match self.current_function {
            Some(ref mut f) if f.variables.contains_key(&node.target) => {
                let var = f.variables.get(&node.target).unwrap();
                self.builder.build_store(*var, value)?;
                return Ok(());
            }
            _ => {}
        };

        if self.global_variables.contains_key(&node.target) {
            let var = self.global_variables.get(&node.target).unwrap();
            self.builder.build_store(*var, value)?;
            Ok(())
        } else {
            Err(anyhow!("Cannot assing value to undefined variable"))
        }
    }

    fn visit_typedef(&mut self, node: &StructTypedef) -> Result<()> {
        let _struct_type = self.context.opaque_struct_type(&node.name);

        Ok(())
    }
}
