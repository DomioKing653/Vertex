use std::collections::HashMap;
use crate::{
    backend::{
        compiler::{
            instructions::Instructions,
            comptime_variable_checker::comptime_value_for_check::ComptimeValueType
        },
        linker::obj_file::ObjFile
    }
};
pub enum SymbolType {
    Function,
    Variable
}

pub struct GlobalSymbols{
    pub symbols:HashMap<String,Symbol>
}

pub struct Symbol{
    pub symbol_value_type:ComptimeValueType,
    pub symbol_type:SymbolType,
    pub is_constant:bool,
    pub tag:String
}



pub struct Linker;

impl Linker {

    pub fn link(objects: Vec<ObjFile>) -> Vec<Instructions> {

        let mut program: Vec<Instructions> = Vec::new();
        let mut offset: usize = 0;

        for obj in objects {

            let mut patched = Vec::new();

            for instr in obj.instructions {

                let new_instr = match instr {

                    Instructions::Jump(addr) => {
                        Instructions::Jump(addr + offset)
                    }

                    Instructions::JumpIfTrue(addr) => {
                        Instructions::JumpIfTrue(addr + offset )
                    }

                    Instructions::JumpIfFalse(addr) => {
                        Instructions::JumpIfFalse(addr + offset)
                    }

                    other => other
                };

                patched.push(new_instr);
            }

            offset += patched.len();
            program.extend(patched);
        }

        program
    }
}