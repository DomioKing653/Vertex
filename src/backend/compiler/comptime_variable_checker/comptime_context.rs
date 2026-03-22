use crate::backend::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType;
use crate::backend::compiler::functions_compiler_context::CompileTimeFunctionForCheck;
use crate::backend::errors::compiler::compiler_errors::CompileError;
use crate::backend::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType::{
    Bool, Int, StringValue, Void, Float
};
use crate::backend::errors::compiler::compiler_errors::CompileError::UndefinedType;
use std::collections::HashMap;

pub struct CompileContext {
    pub variables: HashMap<String, ComptimeVariable>,
    pub functions: Vec<HashMap<String, CompileTimeFunctionForCheck>>,
    pub scopes: Vec<HashMap<String, ComptimeVariable>>,
    current_variable_tag: String,
    pub types:Vec<String>
}
impl CompileContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: vec![HashMap::new()],
            scopes: vec![HashMap::new()],
            current_variable_tag: "default".into(),
            types:Vec::new()
        }
    }
    pub fn add_type(&mut self,type_to_add:String)->Result<(),CompileError>{
        if !self.types.contains(&type_to_add) {
            self.types.push(type_to_add);
            return Ok(());
        }
        return Err(CompileError::TypeAlredyExists { name_of_type: type_to_add });
    }
    pub fn get_type(&self,type_to_identify: &str) -> Result<ComptimeValueType, CompileError> {
        match type_to_identify {
            "int" => Ok(Int),
            "string" => Ok(StringValue),
            "bool" => Ok(Bool),
            "void" => Ok(Void),
            "flt" => Ok(Float),
            _ => Err(UndefinedType {
                undefined_type: type_to_identify.to_string(),
            }),
        }
    }
    pub fn exit_scope(&mut self) {
        self.scopes
            .pop()
            .expect("Fatal error: stack underflow at compilation!");

    }
    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    pub fn add_variable(
        &mut self,
        name: String,
        variable: ComptimeVariable,
    ) -> Result<(), CompileError> {
        let current_scope = self.scopes.last_mut().unwrap();
        if current_scope.contains_key(&name) {
            return Err(CompileError::VariableRecreation { name });
        } else {
            current_scope.insert(name, variable);
            Ok(())
        }
    }
    pub fn get_variable(&self, name: &str) -> Option<&ComptimeVariable> {
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Some(v);
            }
        }
        None
    }
    pub fn add_function(
        &mut self,
        name: String,
        fnc: CompileTimeFunctionForCheck,
    ) -> Result<(), CompileError> {
        let curren_fnc_scope = self.functions.last_mut().unwrap();
        if curren_fnc_scope.contains_key(&name) {
            return Err(CompileError::FunctionAlredyExists { name });
        } else {
            curren_fnc_scope.insert(name, fnc);
            Ok(())
        }
    }
    pub fn get_fn(&mut self, name: &str) -> Result<CompileTimeFunctionForCheck, CompileError> {
        self.functions
            .last_mut()
            .unwrap()
            .get(name)
            .cloned()
            .ok_or(CompileError::UnknownFunction {
                name: name.to_string(),
            })
    }
}

pub struct ComptimeVariable {
    pub value_type: ComptimeValueType,
    pub tag: String,
    pub is_const: bool,
}
