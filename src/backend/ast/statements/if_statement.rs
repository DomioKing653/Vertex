use crate::backend::{
    compiler::{ byte_code::{Compilable, Compiler, indent_fn}, comptime_variable_checker::comptime_value_for_check::ComptimeValueType, instructions::Instructions},
    errors::compiler::compiler_errors::CompileError,
};
use std::fmt::Debug;

#[derive(Clone)]
pub struct IfStatement {
    pub then_branch: Vec<Box<dyn Compilable>>,
    pub condition: Box<dyn Compilable>,
    pub else_branch: Option<Vec<Box<dyn Compilable>>>,
}

impl Compilable for IfStatement {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        let cond_type = self.condition.compile(compiler)?;
        if cond_type != ComptimeValueType::Bool {
            return Err(CompileError::TypeMismatch {
                expected: ComptimeValueType::Bool,
                found: cond_type,
            });
        }

        let jump_if_false_pos = compiler.out.len();
        compiler.out.push(Instructions::JumpIfFalse(0)); // Placeholder for jump instruction
        compiler.context.enter_scope();
        for stmt in &mut self.then_branch {
            stmt.compile(compiler)?;
        }
        compiler.exit_scope();
        let jump_end_pos = compiler.out.len();
        compiler.out.push(Instructions::Jump(0)); // Placeholder for end jump instruction

        let else_start = compiler.out.len();
        compiler.out[jump_if_false_pos] = Instructions::JumpIfFalse(else_start); // If false, jump to else_start
        compiler.context.enter_scope();
        if let Some(else_branch) = &mut self.else_branch {
            for stmt in else_branch {
                stmt.compile(compiler)?;
            }
        }
        compiler.exit_scope();

        let end = compiler.out.len();
        compiler.out[jump_end_pos] = Instructions::Jump(end); // Jump to end

        Ok(ComptimeValueType::Void) // void
    }
    fn fmt_with_indent(&self, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
        writeln!(f, "{}if(...)", indent_fn(indent))?;
        let mut i = 0;
        while i < self.then_branch.len() {
            self.then_branch[i].fmt_with_indent(f, indent)?;
            i += 1;
        }
        Ok(())
    }
    fn add_to_lookup(&self, compiler: &mut Compiler) -> Result<(), CompileError> {
        Ok(())
    }

    fn add_to_type_check(&self, compiler: &mut Compiler) -> Result<(), CompileError> {
        Ok(())
    }

    fn my_type(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        Ok(ComptimeValueType::Void)
    }
}

impl Debug for IfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
