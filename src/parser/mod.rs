use crate::{
    Lexer,
    ast::{self, AstNode},
};
use anyhow::{Result, anyhow};

mod declarations;
mod instructions;
mod operators;
mod utils;

pub trait Parser {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>>;
}

// <program> := {{typedef}{<declaration>}{<function>}}
pub struct Program;

// <typedef> := "struct"<identifier>"{"{<identifier>":"<type>";"}"}
struct Typedef;

// <declaration> := "let"<identifier>":"<type>["="<or>];
struct Declaration;

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

//<assignment> := <identifier>"="<or>";"
struct Assignment;

// <if> := "if" <expression><block>["else"<block>]
struct If;

// <while> := "while"<or><block>
struct While;

// <return> := "return"<or>";
struct Return;

// <or> = <and>
//       | <or>"||"<and>
pub struct Or;

// <and> = <relation>
//       | <and>"&&"<relation>
pub struct And;

// <equality> = <relation>"=="<relation>
//            | <relation>"!="<relation>
pub struct Equality;

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
//           | <member_access>
//           | <function_call>
//           | "("<or>")"
struct Factor;

// <function_call> := <identifier>"(" [<or> {","<or>} ] ")"
struct FunctionCall;

// <member_access> := <identifier>"."<identifier>
struct MemberAccess;

// <parameter> := <identifier>":"<type>

// <identifier> := [a-zA-Z_][a-zA-Z0-9_]*

// <type> := "int"

impl Parser for Program {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let mut program = ast::Program::new();
        while lexer.peek().is_some() {
            if lexer.peek_and(|s| s == "let") {
                program.add_node(Declaration::parse(lexer)?);
            } else if lexer.peek_and(|s| s.starts_with("fn")) {
                program.add_node(Function::parse(lexer)?);
            } else if lexer.peek_and(|s| s == "struct") {
                program.add_node(Typedef::parse(lexer)?);
            } else {
                return Err(anyhow!("Unexpected token: \"{}\"", lexer.peek().unwrap()));
            }
        }

        Ok(program)
    }
}
