use std::collections::HashMap;

use anyhow::anyhow;

use crate::ast;
use crate::compiler::{AstVisitor, type_checker::*};

struct TypeConverter {
    generator: TypeVariableGenerator,
    context: Context,

    expression: Vec<Expression>,
}

impl TypeConverter {
    pub fn new() -> Self {
        let context = Context::new();
        let mut primitive_mappings = HashMap::new();
        primitive_mappings.insert(
            "intlit".to_string(),
            PolymorphicType::MonomorphicType(MonomorphicType::FunctionApplication(
                "i32".to_string(),
                vec![],
            )),
        );
        primitive_mappings.insert(
            "boollit".to_string(),
            PolymorphicType::MonomorphicType(MonomorphicType::FunctionApplication(
                "b8".to_string(),
                vec![],
            )),
        );
        primitive_mappings.insert(
            "sum".to_string(),
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
            "subtract".to_string(),
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
            "multiply".to_string(),
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
            "divide".to_string(),
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
            "less".to_string(),
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
        primitive_mappings.insert(
            "greater".to_string(),
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
        primitive_mappings.insert(
            "equals".to_string(),
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
        primitive_mappings.insert(
            "not".to_string(),
            PolymorphicType::MonomorphicType(MonomorphicType::FunctionApplication(
                "->".to_string(),
                vec![
                    MonomorphicType::FunctionApplication("b8".to_string(), vec![]),
                    MonomorphicType::FunctionApplication("b8".to_string(), vec![]),
                ],
            )),
        );
        primitive_mappings.insert(
            "and".to_string(),
            PolymorphicType::MonomorphicType(MonomorphicType::FunctionApplication(
                "->".to_string(),
                vec![
                    MonomorphicType::FunctionApplication("b8".to_string(), vec![]),
                    MonomorphicType::FunctionApplication(
                        "->".to_string(),
                        vec![
                            MonomorphicType::FunctionApplication("b8".to_string(), vec![]),
                            MonomorphicType::FunctionApplication("b8".to_string(), vec![]),
                        ],
                    ),
                ],
            )),
        );
        primitive_mappings.insert(
            "or".to_string(),
            PolymorphicType::MonomorphicType(MonomorphicType::FunctionApplication(
                "->".to_string(),
                vec![
                    MonomorphicType::FunctionApplication("b8".to_string(), vec![]),
                    MonomorphicType::FunctionApplication(
                        "->".to_string(),
                        vec![
                            MonomorphicType::FunctionApplication("b8".to_string(), vec![]),
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
}

impl AstVisitor for TypeConverter {
    fn visit(&mut self, node: &ast::Program) -> anyhow::Result<()> {
        todo!()
    }

    fn visit_addition(&mut self, node: &ast::Addition) -> anyhow::Result<()> {
        let left = {
            node.left.accept(self)?;
            self.expression.pop().ok_or(anyhow!("A"))?
        };
        let left = Expression::Application(
            Box::new(Expression::Variable("sum".to_string())),
            Box::new(left),
        );
        let right = {
            node.right.accept(self)?;
            self.expression.pop().ok_or(anyhow!("B"))?
        };

        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));

        Ok(())
    }

    fn visit_subtraction(&mut self, node: &ast::Subtraction) -> anyhow::Result<()> {
        let left = {
            node.left.accept(self)?;
            self.expression.pop().ok_or(anyhow!("A"))?
        };
        let left = Expression::Application(
            Box::new(Expression::Variable("subtraction".to_string())),
            Box::new(left),
        );
        let right = {
            node.right.accept(self)?;
            self.expression.pop().ok_or(anyhow!("B"))?
        };

        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));

        Ok(())
    }

    fn visit_multiplication(&mut self, node: &ast::Multiplication) -> anyhow::Result<()> {
        let left = {
            node.left.accept(self)?;
            self.expression.pop().ok_or(anyhow!("A"))?
        };
        let left = Expression::Application(
            Box::new(Expression::Variable("multiply".to_string())),
            Box::new(left),
        );
        let right = {
            node.right.accept(self)?;
            self.expression.pop().ok_or(anyhow!("B"))?
        };

        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));

        Ok(())
    }

    fn visit_division(&mut self, node: &ast::Division) -> anyhow::Result<()> {
        let left = {
            node.left.accept(self)?;
            self.expression.pop().ok_or(anyhow!("A"))?
        };
        let left = Expression::Application(
            Box::new(Expression::Variable("divide".to_string())),
            Box::new(left),
        );
        let right = {
            node.right.accept(self)?;
            self.expression.pop().ok_or(anyhow!("B"))?
        };

        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));

        Ok(())
    }

    fn visit_integer(&mut self, node: &ast::Integer) -> anyhow::Result<()> {
        self.expression
            .push(Expression::Variable("intlit".to_string()));

        Ok(())
    }

    fn visit_identifier(&mut self, node: &ast::Identifier) -> anyhow::Result<()> {
        todo!()
    }

    fn visit_declaration(&mut self, node: &ast::Declaration) -> anyhow::Result<()> {
        todo!()
    }

    fn visit_block(&mut self, node: &ast::Block) -> anyhow::Result<()> {
        todo!()
    }

    fn visit_function(&mut self, node: &ast::Function) -> anyhow::Result<()> {
        todo!()
    }

    fn visit_if(&mut self, node: &ast::If) -> anyhow::Result<()> {
        todo!()
    }

    fn visit_while(&mut self, node: &ast::While) -> anyhow::Result<()> {
        todo!()
    }

    fn visit_return(&mut self, node: &ast::Return) -> anyhow::Result<()> {
        todo!()
    }

    fn visit_function_call(&mut self, node: &ast::FunctionCall) -> anyhow::Result<()> {
        todo!()
    }

    fn visit_or(&mut self, node: &ast::Or) -> anyhow::Result<()> {
        let left = {
            node.left.accept(self)?;
            self.expression.pop().ok_or(anyhow!("A"))?
        };
        let left = Expression::Application(
            Box::new(Expression::Variable("or".to_string())),
            Box::new(left),
        );
        let right = {
            node.right.accept(self)?;
            self.expression.pop().ok_or(anyhow!("B"))?
        };
        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));
        Ok(())
    }

    fn visit_and(&mut self, node: &ast::And) -> anyhow::Result<()> {
        let left = {
            node.left.accept(self)?;
            self.expression.pop().ok_or(anyhow!("A"))?
        };
        let left = Expression::Application(
            Box::new(Expression::Variable("and".to_string())),
            Box::new(left),
        );
        let right = {
            node.right.accept(self)?;
            self.expression.pop().ok_or(anyhow!("B"))?
        };
        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));
        Ok(())
    }

    fn visit_equality(&mut self, node: &ast::Equality) -> anyhow::Result<()> {
        let left = {
            node.left.accept(self)?;
            self.expression.pop().ok_or(anyhow!("A"))?
        };
        let left = Expression::Application(
            Box::new(Expression::Variable("equals".to_string())),
            Box::new(left),
        );
        let right = {
            node.right.accept(self)?;
            self.expression.pop().ok_or(anyhow!("B"))?
        };
        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));
        Ok(())
    }

    fn visit_inequality(&mut self, node: &ast::Inequality) -> anyhow::Result<()> {
        let left = {
            node.left.accept(self)?;
            self.expression.pop().ok_or(anyhow!("A"))?
        };
        let left = Expression::Application(
            Box::new(Expression::Variable("not".to_string())),
            Box::new(Expression::Application(
                Box::new(Expression::Variable("equals".to_string())),
                Box::new(left),
            )),
        );
        let right = {
            node.right.accept(self)?;
            self.expression.pop().ok_or(anyhow!("B"))?
        };
        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));
        Ok(())
    }

    fn visit_greater(&mut self, node: &ast::Greater) -> anyhow::Result<()> {
        let left = {
            node.left.accept(self)?;
            self.expression.pop().ok_or(anyhow!("A"))?
        };
        let left = Expression::Application(
            Box::new(Expression::Variable("greater".to_string())),
            Box::new(left),
        );
        let right = {
            node.right.accept(self)?;
            self.expression.pop().ok_or(anyhow!("B"))?
        };
        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));
        Ok(())
    }

    fn visit_lesser(&mut self, node: &ast::Lesser) -> anyhow::Result<()> {
        let left = {
            node.left.accept(self)?;
            self.expression.pop().ok_or(anyhow!("A"))?
        };
        let left = Expression::Application(
            Box::new(Expression::Variable("less".to_string())),
            Box::new(left),
        );
        let right = {
            node.right.accept(self)?;
            self.expression.pop().ok_or(anyhow!("B"))?
        };
        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));
        Ok(())
    }

    fn visit_greater_equal(&mut self, node: &ast::GreaterEqual) -> anyhow::Result<()> {
        let left = {
            node.left.accept(self)?;
            self.expression.pop().ok_or(anyhow!("A"))?
        };
        let left = Expression::Application(
            Box::new(Expression::Variable("not".to_string())),
            Box::new(Expression::Application(
                Box::new(Expression::Variable("less".to_string())),
                Box::new(left),
            )),
        );
        let right = {
            node.right.accept(self)?;
            self.expression.pop().ok_or(anyhow!("B"))?
        };
        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));
        Ok(())
    }

    fn visit_lesser_equal(&mut self, node: &ast::LesserEqual) -> anyhow::Result<()> {
        let left = {
            node.left.accept(self)?;
            self.expression.pop().ok_or(anyhow!("A"))?
        };
        let left = Expression::Application(
            Box::new(Expression::Variable("not".to_string())),
            Box::new(Expression::Application(
                Box::new(Expression::Variable("greater".to_string())),
                Box::new(left),
            )),
        );
        let right = {
            node.right.accept(self)?;
            self.expression.pop().ok_or(anyhow!("B"))?
        };
        self.expression
            .push(Expression::Application(Box::new(left), Box::new(right)));
        Ok(())
    }

    fn visit_assignment(&mut self, node: &ast::Assignment) -> anyhow::Result<()> {
        todo!()
    }

    fn visit_typedef(&mut self, node: &ast::StructTypedef) -> anyhow::Result<()> {
        todo!()
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
                            Expression::Variable(name) if name == "sum" => {}
                            _ => panic!("Expected function application to 'sum'"),
                        }
                        match &**arg2 {
                            Expression::Variable(name) if name == "intlit" => {}
                            _ => panic!("Expected argument to be 'intlit'"),
                        }
                    }
                    _ => panic!("Expected function application"),
                }
                match &**arg {
                    Expression::Variable(name) if name == "intlit" => {}
                    _ => panic!("Expected argument to be 'intlit'"),
                }
            }
            _ => panic!("Expected function application"),
        }

        let mut generator = TypeVariableGenerator::new();
        let (_, mono_type) = w(&converter.context, &infer_tree, &mut generator).unwrap();

        match &mono_type {
            MonomorphicType::FunctionApplication(name, args) => {
                if name == "i32" && args.len() == 0 {}
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
                            Expression::Variable(name) if name == "sum" => {}
                            _ => panic!("Expected function application to 'sum'"),
                        }
                        match &**arg2 {
                            Expression::Variable(name) if name == "intlit" => {}
                            _ => panic!("Expected argument to be 'intlit'"),
                        }
                    }
                    _ => panic!("Expected function application"),
                }
                match &**arg {
                    Expression::Variable(name) if name == "intlit" => {}
                    _ => panic!("Expected argument to be 'intlit'"),
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
                if name == "i32" && args.len() == 0 {}
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
                            Expression::Variable(name) if name == "equals" => {}
                            _ => panic!("Expected function application to 'equals'"),
                        }
                        match &**arg2 {
                            Expression::Variable(name) if name == "intlit" => {}
                            _ => panic!("Expected argument to be 'intlit'"),
                        }
                    }
                    _ => panic!("Expected function application"),
                }
                match &**arg {
                    Expression::Variable(name) if name == "intlit" => {}
                    _ => panic!("Expected argument to be 'intlit'"),
                }
            }
            _ => panic!("Expected function application"),
        }

        let mut generator = TypeVariableGenerator::new();
        let (_, mono_type) = w(&converter.context, &infer_tree, &mut generator).unwrap();

        match &mono_type {
            MonomorphicType::FunctionApplication(name, args) => {
                if name == "b8" && args.len() == 0 {}
            }
            _ => panic!("Expected function type from i32 to b8"),
        }
    }
}
