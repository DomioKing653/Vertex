use crate::backend::{
    ast::functions::args_node::FunctionArgs,
    compiler::{
        byte_code::Compilable,
        comptime_variable_checker::comptime_value_for_check::ComptimeValueType,
    },
};
#[derive(Clone)]
pub struct CompileTimeFunctionForCheck {
    pub return_type: ComptimeValueType,
    pub is_pub: bool,
    pub body: Vec<Box<dyn Compilable>>,
    pub args: Vec<FunctionArgs>,
}
