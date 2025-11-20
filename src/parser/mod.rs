use crate::{
    Lexer,
    ast::{
        self, Addition, AstNode, BinaryAstNode, Division, Integer, LiteralAstNode, Multiplication,
        Subtraction,
    },
};
use anyhow::{Result, anyhow};

pub trait Parser {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>>;
}
// <program> := {{<declaration>}{<function>}}
pub struct Program;

// <function> := "fn"<identifier>"("[{<parameter>","}<parameter>]")[":"<type>]<block>
struct Function;

// <block> := "{"{<instruction>}"}"
struct Block;

// <instruction> := <declaration>
//                | <assignment> TODO: stesso ss come function_call
//                | <if>
//                | <while>
//                | <return>
//                | <function_call>";"
struct Instruction;

// <return> := "return"<or>";
struct Return;

// <if> := "if" <expression><block>["else"<block>]
struct If;

// <while> := "while"<or><block>
struct While;

// <declaration> := "let"<identifier>":"<type>["="<or>];
struct Declaration;

//<assignment> := <identifier>"="<or>";"
struct Assignment;

// <or> = <and>
//       | <or>"||"<and>
pub struct Or;

// <and> = <relation>
//       | <and>"&&"<relation>
pub struct And;

// <relation> = <expression>
//            | <relation>"<"<expression>
//            | <relation>">"<expression>
//            | <relation>"<="<expression>
//            | <relation>">="<expression>
pub struct Relation;

// <expression> := <term>"+"<expression>
//               | <term>"-"<expression>
struct Expression;

// <term> := <factor>"*"<factor>
//         | <factor>"/"<factor>
struct Term;

// <factor> := <integer>
//           | <identifier> TODO: stesso starter set
//           | <function_call>
//           | "("<or>")"
struct Factor;

// <function_call> := <identifier>"(" [<or> {","<or>} ] ")"
struct FunctionCall;

impl Parser for Program {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let mut program = ast::Program::new();
        while lexer.peek().is_some() {
            if lexer.peek_and(|s| s == "let") {
                program.add_node(Declaration::parse(lexer)?);
            } else if lexer.peek_and(|s| s.starts_with("fn")) {
                program.add_node(Function::parse(lexer)?);
            } else {
                return Err(anyhow!("Unexpected token: \"{}\"", lexer.peek().unwrap()));
            }
        }

        Ok(program)
    }
}

impl Parser for Function {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if !lexer.next_token().is_some_and(|s| s == "fn") {
            return Err(anyhow!("Error parsing function: missing \"fn\""));
        }

        let name = parse_identifier(lexer)?;
        if !lexer.peek_and(|s| s.starts_with("(")) {
            return Err(anyhow!("Error parsing function: missing \'(\'"));
        }
        lexer.pop_char();

        let mut parameters = Vec::new();
        while !lexer.peek_and(|s| s.starts_with(")")) {
            let (name, type_name) = parse_parameter(lexer)?;
            parameters.push(ast::Parameter { name, type_name });
        }
        lexer.pop_char();

        let mut return_type = "void".to_string();
        if lexer.peek_and(|s| s == ":") {
            let _ = lexer.next_token();
            match lexer.next_token() {
                Some(type_name) => {
                    return_type = type_name;
                }
                None => {
                    return Err(anyhow!("Error parsing function: missing return type"));
                }
            }
        }

        let body = Block::parse(lexer)?;
        Ok(ast::Function::new(name, parameters, return_type, body))
    }
}

impl Parser for Block {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        match lexer.peek() {
            Some(item) if item == "{" => {
                lexer.pop_char();
                let mut block = ast::Block::new();
                while !lexer.peek_and(|s| s.starts_with("}")) {
                    block.add_node(Instruction::parse(lexer)?);
                }

                if !lexer.peek_and(|s| s.starts_with("}")) {
                    return Err(anyhow!("Failed to parse block: missing '}}'"));
                }
                lexer.pop_char();

                return Ok(block);
            }
            Some(item) if item == "let" => Declaration::parse(lexer),
            _ => {
                return Err(anyhow!("Unexpected token: \"{}\"", lexer.peek().unwrap()));
            }
        }
    }
}

impl Parser for Instruction {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        match lexer.peek() {
            Some(item) if item == "let" => Declaration::parse(lexer),
            Some(item) if item == "if" => If::parse(lexer),
            Some(item) if item == "while" => While::parse(lexer),
            Some(item) if item == "return" => Return::parse(lexer),
            Some(_) => {
                if lexer.peek_and_n(2, |p| p.contains("=")) {
                    Assignment::parse(lexer)
                } else {
                    let call = FunctionCall::parse(lexer)?;
                    if lexer.next_token().is_some_and(|s| s == ";") {
                        Ok(call)
                    } else {
                        Err(anyhow!("Error parsing function call: missing \";\""))
                    }
                }
            }
            None => Err(anyhow!("Error parsing instruction: unexpected EOF")),
        }
    }
}

impl Parser for Return {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if !lexer.next_token().is_some_and(|s| s == "return") {
            return Err(anyhow!(
                "Error parsing return statement: missing \"return\""
            ));
        }

        let expression = Or::parse(lexer)?;
        if !lexer.next_token().is_some_and(|s| s == ";") {
            return Err(anyhow!("Error parsing return statement: missing \";\""));
        }

        Ok(ast::Return::new(expression))
    }
}

impl Parser for If {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if !lexer.next_token().is_some_and(|s| s == "if") {
            return Err(anyhow!("Error parsing if statement: missing \"if\""));
        }

        let expression = Or::parse(lexer)?;
        let then_block = Block::parse(lexer)?;

        let else_block = if lexer.peek_and(|s| s == "else") {
            let _ = lexer.next_token();
            Some(Block::parse(lexer)?)
        } else {
            None
        };

        Ok(ast::If::new(expression, then_block, else_block))
    }
}

impl Parser for While {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if !lexer.next_token().is_some_and(|s| s == "while") {
            return Err(anyhow!("Error parsing while statement: missing \"while\""));
        }

        let expression = Or::parse(lexer)?;
        let block = Block::parse(lexer)?;
        Ok(ast::While::new(expression, block))
    }
}

impl Parser for Declaration {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if !lexer.next_token().is_some_and(|s| s == "let") {
            return Err(anyhow!("Error parsing declaration: missing \"let\""));
        }

        let identifier = parse_identifier(lexer)?;
        if !lexer.next_token().is_some_and(|s| s == ":") {
            return Err(anyhow!("Error parsing declaration: missing \":\""));
        }

        let var_type = parse_identifier(lexer)?;
        if var_type != "i32" {
            return Err(anyhow!(
                "Error parsing declaration: unsupported type \"{}\"",
                var_type
            ));
        }

        let expression = if lexer.peek_and(|s| s == "=") {
            lexer.next_token();
            Or::parse(lexer)?
        } else {
            Integer::new(0)
        };

        if !lexer.next_token().is_some_and(|s| s == ";") {
            return Err(anyhow!("Error parsing declaration: missing \";\""));
        }

        Ok(ast::Declaration::new(identifier, var_type, expression))
    }
}

impl Parser for Assignment {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let target = parse_identifier(lexer)?;
        if !lexer.peek_and(|s| s.starts_with("=")) {
            return Err(anyhow!("Error parsing assignment: missing \"=\""));
        }
        lexer.pop_char();

        let value = Or::parse(lexer)?;
        if !lexer.peek_and(|s| s.starts_with(";")) {
            return Err(anyhow!("Error parsing assignment: missing \";\""));
        }
        lexer.pop_char();

        Ok(ast::Assignment::new(target, value))
    }
}

impl Parser for Or {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let mut left = And::parse(lexer)?;
        while lexer.peek_and(|s| s == "||") {
            match lexer.peek() {
                Some(val) if val == "||" => {
                    let _ = lexer.next_token();
                    let right = And::parse(lexer)?;
                    left = ast::Or::new(left, right);
                }
                _ => {
                    return Err(anyhow!(
                        "Error parsing expression: unexpected token \"{:?}\"",
                        lexer.peek()
                    ));
                }
            }
        }

        Ok(left)
    }
}

impl Parser for And {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let mut left = Relation::parse(lexer)?;
        while lexer.peek_and(|s| s == "&&") {
            match lexer.peek() {
                Some(val) if val == "&&" => {
                    let _ = lexer.next_token();
                    let right = Relation::parse(lexer)?;
                    left = ast::And::new(left, right);
                }
                _ => {
                    return Err(anyhow!(
                        "Error parsing expression: unexpected token \"{:?}\"",
                        lexer.peek()
                    ));
                }
            }
        }

        Ok(left)
    }
}

impl Parser for Relation {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let mut left = Expression::parse(lexer)?;
        while lexer.peek_and(|s| s == "<" || s == ">" || s == "<=" || s == ">=") {
            match lexer.peek() {
                Some(val) if val == "<" => {
                    let _ = lexer.next_token();
                    let right = Expression::parse(lexer)?;
                    left = ast::Lesser::new(left, right);
                }
                Some(val) if val == ">" => {
                    let _ = lexer.next_token();
                    let right = Expression::parse(lexer)?;
                    left = ast::Greater::new(left, right);
                }
                Some(val) if val == "<=" => {
                    let _ = lexer.next_token();
                    let right = Expression::parse(lexer)?;
                    left = ast::LesserEqual::new(left, right);
                }
                Some(val) if val == ">=" => {
                    let _ = lexer.next_token();
                    let right = Expression::parse(lexer)?;
                    left = ast::GreaterEqual::new(left, right);
                }
                _ => {
                    return Err(anyhow!(
                        "Error parsing expression: unexpected token \"{:?}\"",
                        lexer.peek()
                    ));
                }
            }
        }

        Ok(left)
    }
}

impl Parser for Expression {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let mut left = Term::parse(lexer)?;
        while lexer.peek_and(|s| s == "+" || s == "-") {
            match lexer.peek() {
                Some(val) if val == "+" => {
                    let _ = lexer.next_token();
                    let right = Term::parse(lexer)?;
                    left = Addition::new(left, right);
                }
                Some(val) if val == "-" => {
                    let _ = lexer.next_token();
                    let right = Term::parse(lexer)?;
                    left = Subtraction::new(left, right);
                }
                _ => {
                    return Err(anyhow!(
                        "Error parsing expression: unexpected token \"{:?}\"",
                        lexer.peek()
                    ));
                }
            }
        }

        Ok(left)
    }
}

impl Parser for Term {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let mut left = Factor::parse(lexer)?;
        while lexer.peek_and(|s| s == "*" || s == "/") {
            match lexer.peek() {
                Some(val) if val == "*" => {
                    let _ = lexer.next_token();
                    let right = Term::parse(lexer)?;
                    left = Multiplication::new(left, right);
                }
                Some(val) if val == "/" => {
                    let _ = lexer.next_token();
                    let right = Term::parse(lexer)?;
                    left = Division::new(left, right);
                }
                _ => {
                    return Err(anyhow!(
                        "Failed to parse expression: unexpected token \"{:?}\"",
                        lexer.peek()
                    ));
                }
            }
        }

        Ok(left)
    }
}

impl Parser for Factor {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if lexer.peek_and(|s| s.starts_with("(")) {
            lexer.pop_char();
            let expression = Or::parse(lexer);
            if !lexer.peek_and(|s| s.starts_with(")")) {
                return Err(anyhow!("Failed to parse expression: missing \')\'"));
            }
            lexer.pop_char();
            return expression;
        }

        if lexer.peek_and(|s| s.chars().next().unwrap().is_numeric()) {
            if let Some(token) = lexer.next_while(|c| c.is_numeric()) {
                Ok(Integer::new(token.parse()?))
            } else {
                Err(anyhow!(
                    "Failed to parse expression: expected integer literal"
                ))
            }
        } else {
            if lexer.peek_and(|s| s.contains("(")) {
                return FunctionCall::parse(lexer);
            } else {
                let identifier = parse_identifier(lexer)?;
                Ok(ast::Identifier::new(identifier))
            }
        }
    }
}

impl Parser for FunctionCall {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let name = parse_identifier(lexer)?;
        if !lexer.peek_and(|s| s.starts_with("(")) {
            return Err(anyhow!("Error parsing function call: missing \'(\'"));
        }
        lexer.pop_char();

        let mut arguments = Vec::new();
        while !lexer.peek_and(|s| s.starts_with(")")) {
            let argument = Or::parse(lexer)?;
            arguments.push(argument);

            if lexer.peek_and(|s| s == ",") {
                lexer.next_token();
            }
        }
        lexer.pop_char();

        Ok(ast::FunctionCall::new(name, arguments))
    }
}

fn parse_identifier(lexer: &mut Lexer) -> Result<String> {
    if let Some(token) = lexer.next_while(|c| c.is_alphanumeric() || c == '_') {
        Ok(token)
    } else {
        Err(anyhow!(
            "Failed to parse identifier: expected alphanumeric characters or '_'"
        ))
    }
}

fn parse_parameter(lexer: &mut Lexer) -> Result<(String, String)> {
    let name = parse_identifier(lexer)?;
    if !lexer.next_token().is_some_and(|s| s == ":") {
        return Err(anyhow!("Error parsing parameter: missing \":\""));
    }
    let type_name = parse_identifier(lexer)?;

    Ok((name, type_name))
}
