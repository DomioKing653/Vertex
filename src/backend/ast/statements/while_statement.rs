use std::fmt::Debug;

use crate::backend::{
    compiler::{
        byte_code::{self, Compilable}, comptime_variable_checker::comptime_value_for_check::ComptimeValueType::{self}, instructions::Instructions
    },
    errors::compiler::compiler_errors::CompileError,
};
use crate::backend::compiler::byte_code::Compiler;

#[derive(Clone)]
pub struct WhileStatement {
    pub condition: Box<dyn Compilable>,
    pub body: Vec<Box<dyn Compilable>>,
}

impl Compilable for WhileStatement {
    fn compile(
        &mut self,
        compiler: &mut byte_code::Compiler,
    ) -> Result<ComptimeValueType, CompileError> {
        let cond_type = self.condition.compile(compiler)?;
        if cond_type != ComptimeValueType::Bool {
            return Err(CompileError::TypeMismatch {
                expected: ComptimeValueType::Bool,
                found: cond_type,
            });
        }
        let jump_if_false_pos = compiler.out.len();
        compiler.out.push(Instructions::JumpIfFalse(0));
        let statements_start = compiler.out.len();
        compiler.context.enter_scope();
        for statement in &mut self.body {
            statement.compile(compiler)?;
        }
        compiler.context.exit_scope();
        self.condition.compile(compiler)?;
        compiler
            .out
            .push(Instructions::JumpIfTrue(statements_start));
        compiler.out[jump_if_false_pos] = Instructions::JumpIfFalse(compiler.out.len());
        Ok(ComptimeValueType::Void)
    }
    fn fmt_with_indent(
        &self,
        _f: &mut std::fmt::Formatter<'_>,
        _indent: usize,
    ) -> std::fmt::Result {
        writeln!(_f, "if")
    }
    fn add_to_lookup(&self, compiler: &mut Compiler) -> Result<(), CompileError> {
        Ok(())

    }
    fn my_type(&self,compiler: &mut Compiler) -> ComptimeValueType {
        ComptimeValueType::Void
    }
    fn add_to_type_check(&self, compiler: &mut Compiler) -> Result<(), CompileError> {
        Ok(())
    }
}

impl Debug for WhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WhileStatement")
            .field("condition", &self.condition)
            .field("body", &self.body)
            .finish()
    }
}
