use crate::backend::compiler::byte_code::{Compilable, Compiler};

pub struct Linker{
    pub ast:Vec<Vec<Compilable>>,
    pub compiler: Compiler
}
