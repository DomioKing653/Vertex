use crate::backend::{
    compiler::{
        comptime_variable_checker::comptime_value_for_check::ComptimeValueType,
        instructions::Instructions::{self, Halt},
    },
    linker::{obj_file::ObjFile, patch_objs, sort_objs::sort_objs_bfs},
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
        // Sort objs
        let mut program: Vec<Instructions> = Vec::new();
        let sorted_objects = sort_objs_bfs(objects.clone()).unwrap();
        // Patch jump adresses
        patch_objs::patch_objs_jumps(sorted_objects, &mut program);
        program.push(Halt); // Final Halt of a program
        program
    }
}
