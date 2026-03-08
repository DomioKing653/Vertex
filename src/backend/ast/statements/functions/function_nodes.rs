use std::fmt::Debug;

use crate::backend::{
    ast::statements::functions::args_node::FunctionArgs,
    compiler::{
        byte_code::{Compilable, Compiler}, comptime_variable_checker::{comptime_context::CompileContext, comptime_value_for_check::ComptimeValueType}, functions_compiler_context::CompileTimeFunctionForCheck
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
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        println!("{:?}",&self.return_type);
        let return_type = CompileContext::get_type(&self.return_type.clone().unwrap())?;
        let args = self.args.clone(); 
        compiler.context.add_function(
            self.id.clone(),
            CompileTimeFunctionForCheck{
                is_pub:true,
                return_type:return_type.clone(),
                args,
                body:self.body.clone()
          }
        )?;
        Ok(return_type)
        
    }

    fn fmt_with_indent(&self, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {

        Ok(())
    }
    fn add_to_lookup(&self,compiler:&mut Compiler,symbols:&mut crate::backend::linker::link::GlobalSymbols) {
        
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
