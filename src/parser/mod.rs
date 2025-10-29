use crate::ast::AstNode;
use anyhow::Result;

pub mod arithmetic;
pub mod boolean;
pub mod control_flow;
pub mod lvalues;
pub mod rvalues;
mod utils;

pub trait Parser {
    type TNext: Parser;

    fn parse(string: String) -> Result<(Box<dyn AstNode>, String)>;
}

pub type Start = Instruction;

// <instruction> := <assignment>
//                | <block>
//                | <if>
//                | <while>
//                | <or>
pub struct Instruction;

// <block> := '{' {<Instruction>} '}'
pub struct Block;

// <if> := 'if' <expression> <block> ['else' <block>]
pub struct If;

// <while> := 'while' <expression> <block>
pub struct While;

// <declaration> = "let" <identifier>: <type> ["=" <or>] ";"
// <type> = "i32"
pub struct Declaration;

// <identifier> = [a-zA-Z_][a-zA-Z0-9_]*
pub struct Identifier;

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

// <expression> = <term>
//              | <expression>"+"<term>
//              | <expression>"-"<term>
pub struct Expression;

// <term> = <not>
//        | <term>"*"<not>
//        | <term>"/"<not>
pub struct Term;

// <not> = <factor>
//       | "!"<factor>
pub struct Not;

// <term> = <number>
//        | "("<instruction>")"
pub struct Factor;

#[cfg(test)]
mod tests {
    use crate::ast::{arithmetic::*, boolean::*, rvalues::*};

    use super::*;

    #[test]
    fn test_arithmetic() {
        let expression = "12 + 5 / 4".to_owned();
        let (expression, _remainder) = Start::parse(expression).unwrap();

        let expected = Box::new(Addition {
            left: Box::new(Integer { value: 12 }),
            right: Box::new(Division {
                left: Box::new(Integer { value: 5 }),
                right: Box::new(Integer { value: 4 }),
            }),
        });

        assert_eq!(format!("{}", expression), format!("{}", expected));

        let expression = "(12 + 5) / 4".to_owned();
        let (expression, _remainder) = Start::parse(expression).unwrap();

        let expected = Box::new(Division {
            left: Box::new(Addition {
                left: Box::new(Integer { value: 12 }),
                right: Box::new(Integer { value: 5 }),
            }),
            right: Box::new(Integer { value: 4 }),
        });

        assert_eq!(format!("{}", expression), format!("{}", expected));

        let expression = "3 * 4 + 2 * 5".to_owned();
        let (expression, _remainder) = Start::parse(expression).unwrap();

        let expected = Box::new(Addition {
            left: Box::new(Multiplication {
                left: Box::new(Integer { value: 3 }),
                right: Box::new(Integer { value: 4 }),
            }),
            right: Box::new(Multiplication {
                left: Box::new(Integer { value: 2 }),
                right: Box::new(Integer { value: 5 }),
            }),
        });

        assert_eq!(format!("{}", expression), format!("{}", expected));

        let expression = "((3 + 5) * 2) - 4 / 2".to_owned();
        let (expression, _remainder) = Start::parse(expression).unwrap();

        let expected = Box::new(Subtraction {
            left: Box::new(Multiplication {
                left: Box::new(Addition {
                    left: Box::new(Integer { value: 3 }),
                    right: Box::new(Integer { value: 5 }),
                }),
                right: Box::new(Integer { value: 2 }),
            }),
            right: Box::new(Division {
                left: Box::new(Integer { value: 4 }),
                right: Box::new(Integer { value: 2 }),
            }),
        });

        assert_eq!(format!("{}", expression), format!("{}", expected));
    }

    #[test]
    fn test_boolean() {
        let expression = "3 < 5 && 2 >= 1".to_owned();
        let (expression, _remainder) = Start::parse(expression).unwrap();

        let expected = Box::new(LogicalAnd {
            left: Box::new(Less {
                left: Box::new(Integer { value: 3 }),
                right: Box::new(Integer { value: 5 }),
            }),
            right: Box::new(GreaterEqual {
                left: Box::new(Integer { value: 2 }),
                right: Box::new(Integer { value: 1 }),
            }),
        });

        assert_eq!(format!("{}", expression), format!("{}", expected));

        let expression = "!(4 > 2 || 1 <= 0)".to_owned();
        let (expression, _remainder) = Start::parse(expression).unwrap();

        let expected = Box::new(LogicalNot {
            value: Box::new(LogicalOr {
                left: Box::new(Greater {
                    left: Box::new(Integer { value: 4 }),
                    right: Box::new(Integer { value: 2 }),
                }),
                right: Box::new(LessEqual {
                    left: Box::new(Integer { value: 1 }),
                    right: Box::new(Integer { value: 0 }),
                }),
            }),
        });

        assert_eq!(format!("{}", expression), format!("{}", expected));

        let expression = "5 + 3 > 2 * 4 || 1 && !0".to_owned();
        let (expression, _remainder) = Start::parse(expression).unwrap();

        let expected = Box::new(LogicalOr {
            left: Box::new(Greater {
                left: Box::new(Addition {
                    left: Box::new(Integer { value: 5 }),
                    right: Box::new(Integer { value: 3 }),
                }),
                right: Box::new(Multiplication {
                    left: Box::new(Integer { value: 2 }),
                    right: Box::new(Integer { value: 4 }),
                }),
            }),
            right: Box::new(LogicalAnd {
                left: Box::new(Integer { value: 1 }),
                right: Box::new(LogicalNot {
                    value: Box::new(Integer { value: 0 }),
                }),
            }),
        });

        assert_eq!(format!("{}", expression), format!("{}", expected));
    }
}
