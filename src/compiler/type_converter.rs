use crate::{
    ast::{self},
    compiler::{AstVisitor, type_checker::*},
};
use anyhow::{Result, anyhow};
use std::collections::HashMap;

pub struct TypeConverter {
    generator: TypeVariableGenerator,
    context: Context,

    expression: Vec<Expression>,
}

impl TypeConverter {
    pub fn new() -> Self {
        let context = Context::new();
        let mut primitive_mappings = HashMap::new();
        primitive_mappings.insert(
            "i32_lit".to_string(),
            PolymorphicType::MonomorphicType(MonomorphicType::FunctionApplication(
                "i32".to_string(),
                vec![],
            )),
        );
        primitive_mappings.insert(
            "b8_lit".to_string(),
            PolymorphicType::MonomorphicType(MonomorphicType::FunctionApplication(
                "b8".to_string(),
                vec![],
            )),
        );
        primitive_mappings.insert(
            "binary_i32_to_i32".to_string(),
            PolymorphicType::MonomorphicType(MonomorphicType::FunctionApplication(
                "->".to_string(),
                vec![
                    MonomorphicType::FunctionApplication("i32".to_string(), vec![]),
                    MonomorphicType::FunctionApplication(
                        "->".to_string(),
                        vec![
                            MonomorphicType::FunctionApplication("i32".to_string(), vec![]),
                            MonomorphicType::FunctionApplication("i32".to_string(), vec![]),
                        ],
                    ),
                ],
            )),
        );
        primitive_mappings.insert(
            "binary_i32_to_b8".to_string(),
            PolymorphicType::MonomorphicType(MonomorphicType::FunctionApplication(
                "->".to_string(),
                vec![
                    MonomorphicType::FunctionApplication("i32".to_string(), vec![]),
                    MonomorphicType::FunctionApplication(
                        "->".to_string(),
                        vec![
                            MonomorphicType::FunctionApplication("i32".to_string(), vec![]),
                            MonomorphicType::FunctionApplication("b8".to_string(), vec![]),
                        ],
                    ),
                ],
            )),
        );
        let context = context.extend(primitive_mappings);

        Self {
            generator: TypeVariableGenerator::new(),
            context,
            expression: vec![],
        }
    }

    fn primitive_mapping(&self, name: &str) -> Result<Box<Expression>> {
        if self.context.contains_mapping(name) {
            Ok(Box::new(Expression::Variable(name.to_string())))
        } else {
            Err(anyhow!("Primitive mapping not found: {}", name))
        }
    }

    fn mono_to_type(&self, mono_type: &MonomorphicType) -> Result<ast::Type> {
        match mono_type {
            MonomorphicType::FunctionApplication(name, args) if args.is_empty() => {
                match name.as_str() {
                    "i32" => Ok(ast::Type::Integer32),
                    "b8" => Ok(ast::Type::Boolean8),
                    custom_name => Ok(ast::Type::Custom(custom_name.to_string())),
                }
            }
            _ => Err(anyhow!(
                "Cannot convert type variable to AST type: {:?}",
                mono_type
            )),
        }
    }
}

impl AstVisitor for TypeConverter {
    fn visit(&mut self, node: &ast::Program) -> Result<()> {
        for node in &node.nodes {
            node.accept(self)?;
        }

        Ok(())
    }

    fn visit_addition(&mut self, node: &ast::Addition) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            Expression::Application(
                self.primitive_mapping("binary_i32_to_i32")?,
                Box::new(
                    self.expression
                        .pop()
                        .ok_or(anyhow!("Error parsing addition: missing first argument"))?,
                ),
            )
        };
        let right = {
            node.right.accept(self)?;
            self.expression
                .pop()
                .ok_or(anyhow!("Error parsing addition: missing second argument"))?
        };

        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));

        Ok(())
    }

    fn visit_subtraction(&mut self, node: &ast::Subtraction) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            Expression::Application(
                self.primitive_mapping("binary_i32_to_i32")?,
                Box::new(
                    self.expression
                        .pop()
                        .ok_or(anyhow!("Error parsing subtraction: missing first argument"))?,
                ),
            )
        };
        let right = {
            node.right.accept(self)?;
            self.expression.pop().ok_or(anyhow!(
                "Error parsing subtraction: missing second argument"
            ))?
        };

        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));

        Ok(())
    }

    fn visit_multiplication(&mut self, node: &ast::Multiplication) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            Expression::Application(
                self.primitive_mapping("binary_i32_to_i32")?,
                Box::new(self.expression.pop().ok_or(anyhow!(
                    "Error parsing multiplication: missing first argument"
                ))?),
            )
        };
        let right = {
            node.right.accept(self)?;
            self.expression.pop().ok_or(anyhow!(
                "Error parsing multiplication: missing second argument"
            ))?
        };

        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));

        Ok(())
    }

    fn visit_division(&mut self, node: &ast::Division) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            Expression::Application(
                self.primitive_mapping("binary_i32_to_i32")?,
                Box::new(
                    self.expression
                        .pop()
                        .ok_or(anyhow!("Error parsing division: missing first argument"))?,
                ),
            )
        };
        let right = {
            node.right.accept(self)?;
            self.expression
                .pop()
                .ok_or(anyhow!("Error parsing division: missing second argument"))?
        };

        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));

        Ok(())
    }

    fn visit_integer(&mut self, _node: &ast::Integer) -> Result<()> {
        self.expression.push(*self.primitive_mapping("i32_lit")?);

        Ok(())
    }

    fn visit_identifier(&mut self, _node: &ast::Identifier) -> Result<()> {
        Ok(())
    }

    fn visit_declaration(&mut self, node: &ast::Declaration) -> Result<()> {
        match (&node.value, &node.d_type) {
            (Some(expr), Some(declared_type)) => {
                let expr = {
                    expr.accept(self)?;
                    self.expression
                        .pop()
                        .ok_or(anyhow!("Error parsing declaration value"))?
                };

                let mono_type = {
                    let mono_type = self.generator.generate_mono();
                    let sub = m(&self.context, &expr, &mono_type, &mut self.generator).unwrap();
                    sub.apply_mono(&mono_type)
                };

                let inferred_type = self.mono_to_type(&mono_type)?;
                if &inferred_type == declared_type {
                    Ok(())
                } else {
                    Err(anyhow!(
                        "Type mismatch in declaration {}: declared {:?}, inferred {:?}",
                        node.name,
                        declared_type,
                        inferred_type
                    ))
                }
            }
            (Some(expr), None) => {
                let expr = {
                    expr.accept(self)?;
                    self.expression
                        .pop()
                        .ok_or(anyhow!("Error parsing declaration value"))?
                };

                let mono_type = {
                    let mono_type = self.generator.generate_mono();
                    let sub = m(&self.context, &expr, &mono_type, &mut self.generator).unwrap();
                    sub.apply_mono(&mono_type)
                };

                let ast_type = self.mono_to_type(&mono_type)?;

                println!("Inferred type for declaration {}: {ast_type:?}", node.name);

                Ok(())
            }
            (None, Some(declared_type)) => Ok(()),
            (None, None) => Err(anyhow!(
                "Declaration must have either a type annotation or an initial value"
            )),
        }
    }

    fn visit_block(&mut self, node: &ast::Block) -> Result<()> {
        for statement in &node.nodes {
            statement.accept(self)?;
        }

        Ok(())
    }

    fn visit_function(&mut self, node: &ast::Function) -> Result<()> {
        node.body.accept(self)?;

        Ok(())
    }

    fn visit_if(&mut self, node: &ast::If) -> Result<()> {
        node.then_block.accept(self)?;
        if let Some(else_block) = &node.else_block {
            else_block.accept(self)?;
        }

        Ok(())
    }

    fn visit_while(&mut self, node: &ast::While) -> Result<()> {
        node.block.accept(self)?;

        Ok(())
    }

    fn visit_return(&mut self, _node: &ast::Return) -> Result<()> {
        Ok(())
    }

    fn visit_function_call(&mut self, _node: &ast::FunctionCall) -> Result<()> {
        Ok(())
    }

    fn visit_or(&mut self, node: &ast::Or) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            Expression::Application(
                self.primitive_mapping("binary_i32_to_b8")?,
                Box::new(
                    self.expression
                        .pop()
                        .ok_or(anyhow!("Error parsing division: missing first argument"))?,
                ),
            )
        };
        let right = {
            node.right.accept(self)?;
            self.expression
                .pop()
                .ok_or(anyhow!("Error parsing division: missing second argument"))?
        };

        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));

        Ok(())
    }

    fn visit_and(&mut self, node: &ast::And) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            Expression::Application(
                self.primitive_mapping("binary_i32_to_b8")?,
                Box::new(
                    self.expression
                        .pop()
                        .ok_or(anyhow!("Error parsing division: missing first argument"))?,
                ),
            )
        };
        let right = {
            node.right.accept(self)?;
            self.expression
                .pop()
                .ok_or(anyhow!("Error parsing division: missing second argument"))?
        };

        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));

        Ok(())
    }

    fn visit_equality(&mut self, node: &ast::Equality) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            Expression::Application(
                self.primitive_mapping("binary_i32_to_b8")?,
                Box::new(
                    self.expression
                        .pop()
                        .ok_or(anyhow!("Error parsing equality: missing first argument"))?,
                ),
            )
        };
        let right = {
            node.right.accept(self)?;
            self.expression
                .pop()
                .ok_or(anyhow!("Error parsing equality: missing second argument"))?
        };

        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));

        Ok(())
    }

    fn visit_inequality(&mut self, node: &ast::Inequality) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            Expression::Application(
                self.primitive_mapping("binary_i32_to_b8")?,
                Box::new(
                    self.expression
                        .pop()
                        .ok_or(anyhow!("Error parsing inequality: missing first argument"))?,
                ),
            )
        };
        let right = {
            node.right.accept(self)?;
            self.expression
                .pop()
                .ok_or(anyhow!("Error parsing inequality: missing second argument"))?
        };

        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));

        Ok(())
    }

    fn visit_greater(&mut self, node: &ast::Greater) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            Expression::Application(
                self.primitive_mapping("binary_i32_to_b8")?,
                Box::new(
                    self.expression
                        .pop()
                        .ok_or(anyhow!("Error parsing greater: missing first argument"))?,
                ),
            )
        };
        let right = {
            node.right.accept(self)?;
            self.expression
                .pop()
                .ok_or(anyhow!("Error parsing greater: missing second argument"))?
        };

        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));

        Ok(())
    }

    fn visit_lesser(&mut self, node: &ast::Lesser) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            Expression::Application(
                self.primitive_mapping("binary_i32_to_b8")?,
                Box::new(
                    self.expression
                        .pop()
                        .ok_or(anyhow!("Error parsing lesser: missing first argument"))?,
                ),
            )
        };
        let right = {
            node.right.accept(self)?;
            self.expression
                .pop()
                .ok_or(anyhow!("Error parsing lesser: missing second argument"))?
        };

        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));

        Ok(())
    }

    fn visit_greater_equal(&mut self, node: &ast::GreaterEqual) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            Expression::Application(
                self.primitive_mapping("binary_i32_to_b8")?,
                Box::new(self.expression.pop().ok_or(anyhow!(
                    "Error parsing greater equal: missing first argument"
                ))?),
            )
        };
        let right = {
            node.right.accept(self)?;
            self.expression.pop().ok_or(anyhow!(
                "Error parsing greater equal: missing second argument"
            ))?
        };

        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));

        Ok(())
    }

    fn visit_lesser_equal(&mut self, node: &ast::LesserEqual) -> Result<()> {
        let left = {
            node.left.accept(self)?;
            Expression::Application(
                self.primitive_mapping("binary_i32_to_b8")?,
                Box::new(self.expression.pop().ok_or(anyhow!(
                    "Error parsing lesser equal: missing first argument"
                ))?),
            )
        };
        let right = {
            node.right.accept(self)?;
            self.expression.pop().ok_or(anyhow!(
                "Error parsing lesser equal: missing second argument"
            ))?
        };

        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));

        Ok(())
    }

    fn visit_assignment(&mut self, _node: &ast::Assignment) -> Result<()> {
        Ok(())
    }

    fn visit_typedef(&mut self, _node: &ast::StructTypedef) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::AstNode;

    #[test]
    fn test_algorithm_w() {
        let ast = ast::Addition {
            left: Box::new(ast::Integer { value: 5 }),
            right: Box::new(ast::Integer { value: 10 }),
        };

        let mut converter = TypeConverter::new();
        let infer_tree = {
            ast.accept(&mut converter).unwrap();
            converter.expression.pop().unwrap()
        };

        match &infer_tree {
            Expression::Application(func, arg) => {
                match &**func {
                    Expression::Application(func2, arg2) => {
                        match &**func2 {
                            Expression::Variable(name) if name == "binary_i32_to_i32" => {}
                            _ => panic!("Expected function application to 'binary_i32_to_i32'"),
                        }
                        match &**arg2 {
                            Expression::Variable(name) if name == "i32_lit" => {}
                            _ => panic!("Expected argument to be 'i32_lit'"),
                        }
                    }
                    _ => panic!("Expected function application"),
                }
                match &**arg {
                    Expression::Variable(name) if name == "i32_lit" => {}
                    _ => panic!("Expected argument to be 'i32_lit'"),
                }
            }
            _ => panic!("Expected function application"),
        }

        let mut generator = TypeVariableGenerator::new();
        let (_, mono_type) = w(&converter.context, &infer_tree, &mut generator).unwrap();

        match &mono_type {
            MonomorphicType::FunctionApplication(name, args) => {
                if name == "i32" && args.is_empty() {}
            }
            _ => panic!("Expected function type from i32 to i32"),
        }
    }

    #[test]
    fn test_algorithm_m() {
        let ast = ast::Addition {
            left: Box::new(ast::Integer { value: 5 }),
            right: Box::new(ast::Integer { value: 10 }),
        };

        let mut converter = TypeConverter::new();
        let infer_tree = {
            ast.accept(&mut converter).unwrap();
            converter.expression.pop().unwrap()
        };

        match &infer_tree {
            Expression::Application(func, arg) => {
                match &**func {
                    Expression::Application(func2, arg2) => {
                        match &**func2 {
                            Expression::Variable(name) if name == "binary_i32_to_i32" => {}
                            _ => panic!("Expected function application to 'binary_i32_to_i32'"),
                        }
                        match &**arg2 {
                            Expression::Variable(name) if name == "i32_lit" => {}
                            _ => panic!("Expected argument to be 'i32_lit'"),
                        }
                    }
                    _ => panic!("Expected function application"),
                }
                match &**arg {
                    Expression::Variable(name) if name == "i32_lit" => {}
                    _ => panic!("Expected argument to be 'i32_lit'"),
                }
            }
            _ => panic!("Expected function application"),
        }

        let mut generator = TypeVariableGenerator::new();
        let new_type = generator.generate_mono();
        let sub = m(&converter.context, &infer_tree, &new_type, &mut generator).unwrap();

        let mono_type = sub.apply_mono(&new_type);

        match &mono_type {
            MonomorphicType::FunctionApplication(name, args) => {
                if name == "i32" && args.is_empty() {}
            }
            _ => panic!("Expected function type from i32 to i32"),
        }
    }

    #[test]
    fn test_int_to_bool() {
        let ast = ast::Equality {
            left: Box::new(ast::Integer { value: 5 }),
            right: Box::new(ast::Integer { value: 10 }),
        };

        let mut converter = TypeConverter::new();
        let infer_tree = {
            ast.accept(&mut converter).unwrap();
            converter.expression.pop().unwrap()
        };

        match &infer_tree {
            Expression::Application(func, arg) => {
                match &**func {
                    Expression::Application(func2, arg2) => {
                        match &**func2 {
                            Expression::Variable(name) if name == "binary_i32_to_b8" => {}
                            _ => panic!("Expected function application to 'equals'"),
                        }
                        match &**arg2 {
                            Expression::Variable(name) if name == "i32_lit" => {}
                            _ => panic!("Expected argument to be 'intlit'"),
                        }
                    }
                    _ => panic!("Expected function application"),
                }
                match &**arg {
                    Expression::Variable(name) if name == "i32_lit" => {}
                    _ => panic!("Expected argument to be 'intlit'"),
                }
            }
            _ => panic!("Expected function application"),
        }

        let mut generator = TypeVariableGenerator::new();
        let (_, mono_type) = w(&converter.context, &infer_tree, &mut generator).unwrap();

        match &mono_type {
            MonomorphicType::FunctionApplication(name, args) => {
                if name == "b8" && args.is_empty() {}
            }
            _ => panic!("Expected function type from i32 to b8"),
        }
    }
}
