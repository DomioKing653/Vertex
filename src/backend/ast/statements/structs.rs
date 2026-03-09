use std::fmt::Debug;

use crate::backend::compiler::byte_code::Compilable;
use crate::backend::compiler::byte_code::Compiler;
use crate::backend::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType;
use crate::backend::errors::compiler::compiler_errors::CompileError;
use crate::backend::linker::link::GlobalSymbols;

#[derive(Clone)]
pub struct StructVariable{
    pub var_type:String
}

#[derive(Clone)]
pub struct StructDefineNode{
    pub args:Vec<StructVariable>
}

impl Compilable for StructDefineNode{
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType,CompileError>{
        todo!()
    }
    fn fmt_with_indent(&self, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
        todo!()
    }
    fn add_to_lookup(&self, compiler: &mut Compiler) {
        
    }
}
impl Debug for StructDefineNode{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

/*
 * Sruct acces node
 */
