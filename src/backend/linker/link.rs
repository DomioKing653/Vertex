use crate::backend::{
    compiler::{
        comptime_variable_checker::comptime_value_for_check::ComptimeValueType,
        instructions::Instructions::{self, Halt},
    },
    linker::{obj_file::ObjFile, sort_objs::sort_objs_bfs},
};
use std::collections::HashMap;

pub enum SymbolType {
    Function,
    Variable,
}

pub struct GlobalSymbols {
    pub symbols: HashMap<String, Symbol>,
}

pub struct Symbol {
    pub symbol_value_type: Option<ComptimeValueType>,
    pub symbol_type: SymbolType,
    pub is_constant: bool,
    pub tag: String,
}

pub struct Linker;

impl Linker {
    pub fn link(objects: Vec<ObjFile>) -> Vec<Instructions> {
        let mut program: Vec<Instructions> = Vec::new();
        let mut offset: usize = 0;
        let sorted_objects = sort_objs_bfs(objects.clone()).unwrap();

        for obj in sorted_objects {
            let mut patched = Vec::new();

            for instr in obj.instructions {
                let new_instr = match instr {
                    Instructions::Jump(addr) => Instructions::Jump(addr + offset),

                    Instructions::JumpIfTrue(addr) => Instructions::JumpIfTrue(addr + offset),

                    Instructions::JumpIfFalse(addr) => Instructions::JumpIfFalse(addr + offset),

                    other => other,
                };

                patched.push(new_instr);
            }

            offset += patched.len();
            program.extend(patched);
        }

        program.push(Halt);
        program
    }
}
