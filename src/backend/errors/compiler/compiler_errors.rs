use crate::backend::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType;
use thiserror::Error;

//TODO:Update to error handeling with lines and update the error messages.

#[derive(Debug, Error)]
#[derive(Clone)]
pub enum CompileError {
    #[error("[E0001]Unknown macro: {name}")]
    UnknownMacro { name: String },

    #[error("[E0002]Cannot infer type for {name}")]
    CannotInferType { name: String },

    #[error("[E0003]Undefined type: {undefined_type}")]
    UndefinedType { undefined_type: String },

    #[error("[E0004]Type mismatch: expected {expected:?}, found {found:?}")]
    TypeMismatch {
        expected: ComptimeValueType,
        found: ComptimeValueType,
    },

    #[error("[E0005]Invalid binary operation: {op} between {left:?} and {right:?}")]
    InvalidBinaryOp {
        op: &'static str,
        left: ComptimeValueType,
        right: ComptimeValueType,
    },

    #[error("[E0006]Undefined variable: {name}")]
    UndefinedVariable { name: String },

    #[error("[E0007]Variable {name} already exists")]
    VariableRecreation { name: String },

    #[error("[E0008]Cannot have constant without value")]
    ConstantWithoutValue { name: String },
    #[error("[E0009]Cannot reassign constant {name}")]
    ConstReassignment { name: String },
    #[error("[E0010]Wrong macro argument count: expected {expected}, found {found}")]
    WrongMacroArgCount { expected: usize, found: usize },
    #[error("[E0011]Expected printable but found {found:?}")]
    ExpectedPrintable { found: ComptimeValueType },
    #[error("[E0012]Function {name} is already defined")]
    FunctionAlredyExists{name:String},
    #[error("[E0013]Unknown function:{name}")]
    UnknownFunction{name:String},
    #[error("[E0014]Unexpected number of arguments at function {name}: expected {expected} but got {found}")]
    UnexpectedFunctionArguments{name:String,expected:usize,found:usize},
    #[error("[E0015]Type {name_of_type} alredey exists")]
    TypeAlredyExists{name_of_type:String}
}
