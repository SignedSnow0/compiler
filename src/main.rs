use crate::{
    compiler::{NodeCompiler, llvmcompiler::LlvmCompiler},
    parser::{Parser, Start},
    peek::StringUtils,
};
use anyhow::{Result, anyhow};
use inkwell::context::Context;

mod ast;
mod compiler;
mod parser;
mod peek;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err(anyhow!("Usage: cargo build <file>"));
    }

    let file = &args[1];
    let mut input = std::fs::read_to_string(file).unwrap().trim_mut();

    let context = Context::create();
    let mut compiler = LlvmCompiler::new(file, &context);

    while !input.is_empty() {
        let (expression, remainder) = Start::parse(input.trim_mut())?;
        expression.accept(&mut compiler)?;
        let ir = compiler.compile()?;
        std::fs::write(format!("{}.ll", file), ir)?;
        input = remainder.trim_mut();
    }
    Ok(())
}
