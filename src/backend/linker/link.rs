use std::collections::HashMap;

use crate::backend::compiler::{ comptime_variable_checker::comptime_value_for_check::ComptimeValueType};

pub enum SymbolType {
    Function,
    Variable
}

pub struct GlobalSymbols{
    pub symbols:HashMap<String,Symbol>
}

pub struct Symbol{
    pub symbol_value_type:ComptimeValueType,
    pub symbol_type:SymbolType
}


