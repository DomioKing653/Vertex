use std::fmt::Debug;

use crate::backend::ast::functions::args_node::FunctionArgs;
use crate::backend::{
    compiler::{
        byte_code::{Compilable, Compiler},
        comptime_variable_checker::comptime_value_for_check::ComptimeValueType,
        functions_compiler_context::CompileTimeFunctionForCheck,
    },
    errors::compiler::compiler_errors::CompileError,
};

#[derive(Clone)]
pub struct FunctionDefineNode {
    pub args: Vec<FunctionArgs>,
    pub id: String,
    pub body: Vec<Box<dyn Compilable>>,
    pub return_type: Option<String>,
}

impl Compilable for FunctionDefineNode {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        let return_type = compiler
            .context
            .get_type(&self.return_type.clone().unwrap())?;
        Ok(return_type)
    }

    fn fmt_with_indent(&self, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
        Ok(())
    }
    fn add_to_lookup(&self, compiler: &mut Compiler) -> Result<(), CompileError> {
        compiler.context.add_function(
            self.id.clone(),
            CompileTimeFunctionForCheck {
                is_pub: false,
                return_type: compiler
                    .context
                    .get_type(&self.return_type.clone().unwrap())?,
                body: self.body.clone(),
                args: self.args.clone(),
            },
        )?;
        Ok(())
    }

    fn add_to_type_check(&self, compiler: &mut Compiler) -> Result<(), CompileError> {
        compiler.function_types.insert(
            self.id.clone(),
            compiler
                .context
                .get_type(&self.return_type.clone().unwrap())?,
        );
        Ok(())
    }

    fn my_type(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        Ok(ComptimeValueType::Void)
    }
}

impl Debug for FunctionDefineNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionDefineNode")
            .field("args", &self.args)
            .field("id", &self.id)
            .field("body", &self.body)
            .field("return_type", &self.return_type)
            .finish()
    }
}
