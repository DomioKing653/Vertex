use crate::backend::compiler::instructions::Instructions::{ProcessExit, ReadInput, WriteLastOnStack};
use crate::{
    backend::compiler::{
        byte_code::{Compilable, Compiler},
        comptime_variable_checker::comptime_value_for_check::ComptimeValueType::{
            self, Array, Bool, StringValue, Void,Float
        },
        instructions::Instructions::WriteLnLastOnStack,
    },
    backend::errors::compiler::compiler_errors::CompileError::{self, TypeMismatch},
};
use crate::backend::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType::Int;

pub trait Macro {
    fn compile(
        &self,
        out: &mut Compiler,
        args: &mut [Box<dyn Compilable>],
    ) -> Result<ComptimeValueType, CompileError>;
    fn my_type(&self) -> Result<ComptimeValueType, CompileError>;
}

pub struct WriteLnMacro;

impl Macro for WriteLnMacro {
    fn compile(
        &self,
        compiler: &mut Compiler,
        args: &mut [Box<dyn Compilable>],
    ) -> Result<ComptimeValueType, CompileError> {
        for arg in args {
            let value = arg.compile(compiler)?;
            match value {
                StringValue | Int | Float => compiler.out.push(WriteLnLastOnStack),
                Bool => {
                    return Err(CompileError::ExpectedPrintable { found: Bool });
                }
                Void => {
                    return Err(CompileError::ExpectedPrintable { found: Void });
                }
                Array(t) => {
                    return Err(CompileError::ExpectedPrintable { found: Array(t) });
                }
            }
        }
        Ok(Void)
    }
    fn my_type(&self) -> Result<ComptimeValueType, CompileError> {
        Ok(Void)
    }
}

pub struct WriteMacro;

impl Macro for WriteMacro {
    fn compile(
        &self,
        compiler: &mut Compiler,
        args: &mut [Box<dyn Compilable>],
    ) -> Result<ComptimeValueType, CompileError> {
        for arg in args {
            let value = arg.compile(compiler)?;
            match value {
                StringValue | Int | Float => compiler.out.push(WriteLastOnStack),
                Bool => {
                    return Err(CompileError::ExpectedPrintable { found: Bool });
                }
                Void => {
                    return Err(CompileError::ExpectedPrintable { found: Void });
                }
                Array(t) => {
                    return Err(CompileError::ExpectedPrintable { found: Array(t) });
                }
            }
        }
        Ok(Void)
    }
    fn my_type(&self) -> Result<ComptimeValueType, CompileError> {
        Ok(Void)
    }
}

pub struct ProcessExitMacro;

impl Macro for ProcessExitMacro {
    fn compile(
        &self,
        out: &mut Compiler,
        args: &mut [Box<dyn Compilable>],
    ) -> Result<ComptimeValueType, CompileError> {
        if args.len() != 1 {
            return Err(CompileError::WrongMacroArgCount {
                expected: 1,
                found: args.len(),
            });
        } else {
            let value = args[0].compile(out)?;
            match value {
                Int => {
                    out.out.push(ProcessExit);
                    Ok(Void)
                }
                _ => Err(TypeMismatch {
                    expected: Int,
                    found: value,
                }),
            }
        }
    }
    fn my_type(&self) -> Result<ComptimeValueType, CompileError> {
        Ok(Void)
    }
}

pub struct ReadInputMacro;

impl Macro for ReadInputMacro {
    fn compile(
        &self,
        out: &mut Compiler,
        args: &mut [Box<dyn Compilable>],
    ) -> Result<ComptimeValueType, CompileError> {
        if args.len() != 1 {
            return Err(CompileError::WrongMacroArgCount {
                expected: 1,
                found: args.len(),
            });
        } else {
            let value = args[0].compile(out)?;
            match value {
                StringValue => {
                    out.out.push(WriteLastOnStack);
                    out.out.push(ReadInput);
                    return Ok(StringValue);
                }
                _ => Err(TypeMismatch {
                    expected: StringValue,
                    found: value,
                }),
            }
        }
    }
    fn my_type(&self) -> Result<ComptimeValueType, CompileError> {
        Ok(Void)
    }
}
