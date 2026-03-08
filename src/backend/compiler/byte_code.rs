use crate::backend::{
    ast::{
        nodes::{
            ArrayNode, BinaryOpNode, BoolNode, CallType, FloatNode, FunctionCallNode, ImportNode,
            NumberNode, PrefixExpressionNode, ProgramNode, StringNode, VariableAccessNode,
            VariableAssignNode, VariableDefineNode,
        },
        parser::Parser,
    },
    buildin_macros::get_macro::MacroManager,
    compiler::{
         comptime_variable_checker::{
            comptime_context::{CompileContext, ComptimeVariable},
            comptime_value_for_check::ComptimeValueType::{
                self, Array, Bool, Float, Int, StringValue, Void,
            },
        }, functions_compiler_context::CompileTimeFunctionForCheck, instructions::Instructions::{
            self, Add, Div, Halt, LoadVar, Mul, PushBool, PushNumber, PushString, Sub,
        }, optimization::optimze::optimize
    },
    errors::compiler::compiler_errors::CompileError::{self, CannotInferType, TypeMismatch},
    lexer::{
        lexer::Lexer,
        tokens::{
            Token,
            TokenKind::{self, TRUE},
        },
    }, linker::link::{GlobalSymbols, Symbol,SymbolType},
};
use CompileError::ConstantWithoutValue;
use std::{
    fmt::{self, Debug, Formatter},
    fs, process,
};

pub trait CompilableClone {
    fn clone_box(&self) -> Box<dyn Compilable>;
}

impl<T> CompilableClone for T
where
    T: 'static + Compilable + Clone,
{
    fn clone_box(&self) -> Box<dyn Compilable> {
        Box::new(self.clone())
    }
}
pub trait Compilable: Debug + CompilableClone {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError>;
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result;
    fn add_to_lookup(&self,compiler:&mut Compiler,symbols:&mut GlobalSymbols);
}
pub fn indent_fn(n: usize) -> String {
    "  ".repeat(n)
}

impl Clone for Box<dyn Compilable> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub struct Compiler {
    pub context: CompileContext,
    pub out: Vec<Instructions>,
    pub macros: MacroManager,
    pub current_fn: String,
    pub last_return_adress: Option<usize>,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
impl Compiler {
    pub fn new() -> Self {
        Self {
            context: CompileContext::new(),
            out: Vec::new(),
            macros: MacroManager::new(),
            current_fn: "none".into(),
            last_return_adress: None,
        }
    }
    pub fn optimize(&mut self) {
        let code = self.out.clone();
        self.out = optimize(code);
    }
}
impl Compilable for NumberNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        compiler.out.push(PushNumber(self.number as f32));
        Ok(Int)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Number({})", indent_fn(indent), self.number)
    }
    fn add_to_lookup(&self,compiler:&mut Compiler,symbols:&mut GlobalSymbols) {
        
    }

}

impl Compilable for FloatNode {
    fn compile(&self, out: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        out.out.push(PushNumber(self.number));
        Ok(Float)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Float({})", indent_fn(indent), self.number)
    }
    fn add_to_lookup(&self,compiler:&mut Compiler,symbols:&mut GlobalSymbols) {
        
    }
}

impl Compilable for PrefixExpressionNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        todo!()
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        write!(f, "{}{:?}", indent_fn(indent + 1), self.prefix)?;
        self.value.fmt_with_indent(f, 0)
    }
    fn add_to_lookup(&self,compiler:&mut Compiler,symbols:&mut GlobalSymbols) {
        
    }
}
impl Debug for PrefixExpressionNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

impl Compilable for BinaryOpNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        let right = self.left.compile(compiler)?;
        let left = self.right.compile(compiler)?;
        match self.op_tok {
            TokenKind::PLUS => match (&left, &right) {
                (Int, Int) => {
                    compiler.out.push(Add);
                    Ok(Int)
                }
                (Float, Float) | (Int, Float) | (Float, Int) => {
                    compiler.out.push(Add);
                    Ok(Float)
                }
                (StringValue, StringValue) => {
                    compiler.out.push(Add);
                    Ok(StringValue)
                }
                _ => Err(CompileError::InvalidBinaryOp {
                    op: "+",
                    left,
                    right,
                }),
            },
            TokenKind::MINUS => match (&left, &right) {
                (Int, Int) => {
                    compiler.out.push(Sub);
                    Ok(Int)
                }
                (Float, Float) | (Int, Float) | (Float, Int) => {
                    compiler.out.push(Sub);
                    Ok(Float)
                }
                _ => Err(CompileError::InvalidBinaryOp {
                    op: "-",
                    left,
                    right,
                }),
            },
            TokenKind::TIMES => match (&left, &right) {
                (Int, Int) => {
                    compiler.out.push(Mul);
                    Ok(Int)
                }
                (Float, Float) | (Int, Float) | (Float, Int) => {
                    compiler.out.push(Mul);
                    Ok(Float)
                }
                _ => Err(CompileError::InvalidBinaryOp {
                    op: "*",
                    left,
                    right,
                }),
            },
            TokenKind::DIVIDE => match (&left, &right) {
                (Int, Int) => {
                    compiler.out.push(Div);
                    Ok(Int)
                }
                (Float, Float) | (Int, Float) | (Float, Int) => {
                    compiler.out.push(Div);
                    Ok(Float)
                }
                _ => Err(CompileError::InvalidBinaryOp {
                    op: "/",
                    left,
                    right,
                }),
            },
            TokenKind::MODULO => match (&left, &right) {
                (Int, Int) => {
                    compiler.out.push(Instructions::Modulo);
                    Ok(Int)
                }
                _ => Err(CompileError::InvalidBinaryOp {
                    op: "%",
                    left,
                    right,
                }),
            },
            TokenKind::GREATER => match (&left, &right) {
                (Int, Int) | (Float, Float) | (Int, Float) | (Float, Int) => {
                    compiler.out.push(Instructions::GreaterThan);
                    Ok(Bool)
                }
                _ => Err(CompileError::InvalidBinaryOp {
                    op: ">",
                    left,
                    right,
                }),
            },
            TokenKind::LESS => match (&left, &right) {
                (Int, Int) | (Float, Float) | (Int, Float) | (Float, Int) => {
                    compiler.out.push(Instructions::LessThan);
                    Ok(Bool)
                }
                _ => Err(CompileError::InvalidBinaryOp {
                    op: "<",
                    left,
                    right,
                }),
            },
            _ => unreachable!(),
        }
    }

    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}BinaryOp({:?})", indent_fn(indent), self.op_tok)?;
        self.left.fmt_with_indent(f, indent + 2)?;
        self.right.fmt_with_indent(f, indent + 2)?;
        Ok(())
    }
    fn add_to_lookup(&self,compiler:&mut Compiler,symbols:&mut GlobalSymbols) {
        
    }
}

impl Compilable for ProgramNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        for program_node in &self.program_nodes {
            program_node.compile(compiler)?;
        }
        compiler.out.push(Halt);
        Ok(Void)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Program", indent_fn(indent))?;
        for node in &self.program_nodes {
            node.fmt_with_indent(f, indent + 1)?;
        }
        Ok(())
    }
    fn add_to_lookup(&self,compiler:&mut Compiler,symbols:&mut GlobalSymbols) {
        for node in &self.program_nodes{
            node.add_to_lookup(compiler, symbols);
        }

    }
}

impl Compilable for VariableAccessNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        let var = compiler.context.get_variable(&self.variable_name).ok_or(
            CompileError::UndefinedVariable {
                name: self.variable_name.clone(),
            },
        )?;
        compiler.out.push(LoadVar(var.tag.clone()));
        Ok(var.value_type.clone())
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Var({})", indent_fn(indent), self.variable_name)
    }
    fn add_to_lookup(&self,compiler:&mut Compiler,symbols:&mut GlobalSymbols) {
        
    }
}

impl Compilable for StringNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        compiler.out.push(PushString(self.value.clone()));
        Ok(StringValue)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}String({})", indent_fn(indent), self.value)
    }
    fn add_to_lookup(&self,compiler:&mut Compiler,symbols:&mut GlobalSymbols) {
        
    }

}

impl Compilable for VariableDefineNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        if self.is_const && self.value.is_none() {
            return Err(ConstantWithoutValue {
                name: self.var_name.clone(),
            });
        }
        /*
        Type
        */
        let inferred_type = if let Some(value) = &self.value {
            Some(value.compile(compiler)?)
        } else {
            None
        };
        let declared_type = if let Some(t) = &self.value_type {
            Some(CompileContext::get_type(t)?)
        } else {
            None
        };

        let final_type = match (declared_type, inferred_type) {
            (Some(d), Some(i)) if d == i => d,
            (Some(d), Some(i)) => {
                return Err(TypeMismatch {
                    expected: d,
                    found: i,
                });
            }
            (Some(d), None) => {
                match d {
                    StringValue => compiler.out.push(PushString("".to_string())),
                    Int => compiler.out.push(PushNumber(0f32)),
                    Float => compiler.out.push(PushNumber(0f32)),
                    Bool => compiler.out.push(PushBool(false)),
                    Array(t) => {
                        todo!()
                    }
                    Void => {
                        unreachable!()
                    }
                }
                d
            }
            (None, Some(i)) => i,
            (None, None) => {
                return Err(CannotInferType {
                    name: self.var_name.clone(),
                });
            }
        };

        compiler.context.add_variable(
            self.var_name.clone(),
            ComptimeVariable {
                value_type: final_type,
                is_const: self.is_const,
                tag: format!("{}_{}", self.var_name.clone(), compiler.current_fn.clone()),
            },
        )?;
        let tag = compiler
            .context
            .get_variable(&self.var_name)
            .unwrap()
            .tag
            .clone();
        compiler.out.push(Instructions::SaveVar(tag));

        Ok(Void)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        write!(f, "{}var:{:?}=", indent_fn(indent), self.value_type)?;
        if let Some(value) = &self.value {
            value.fmt_with_indent(f, 0)?;
        } else {
            write!(f, "None")?;
        }
        Ok(())
    }
    fn add_to_lookup(&self,compiler:&mut Compiler,symbols:&mut GlobalSymbols) {
       symbols.symbols.insert(self.var_name.clone(), Symbol{
           symbol_value_type:CompileContext::get_type(self.value_type.clone().unwrap_or_else(||"void".to_string()).as_ref()).unwrap(),
           symbol_type:SymbolType::Variable
       }); 
    }
}

impl Compilable for BoolNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        compiler
            .out
            .push(PushBool(if self.value == TRUE { true } else { false }));
        Ok(Bool)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}String({:?})", indent_fn(indent), self.value)
    }
    fn add_to_lookup(&self,compiler:&mut Compiler,symbols:&mut GlobalSymbols) {
        
    }
}

impl Compilable for VariableAssignNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        let (is_const, expected_type) = {
            let var = compiler.context.get_variable(&self.name).ok_or(
                CompileError::UndefinedVariable {
                    name: self.name.clone(),
                },
            )?;
            (var.is_const, var.value_type.clone())
        };
        if is_const {
            return Err(CompileError::ConstReassignment {
                name: self.name.clone(),
            });
        }

        let value_type = self.value.compile(compiler)?;

        if value_type != expected_type {
            return Err(TypeMismatch {
                expected: expected_type,
                found: value_type,
            });
        }
        // SAFETY: At this moment we surely know that the variable will exist and we dont need to
        // do normal 'unwrap'
        unsafe {
            let tag = compiler
                .context
                .get_variable(&self.name)
                .unwrap_unchecked()
                .tag
                .clone();

            compiler.out.push(Instructions::SaveVar(tag));
        }

        Ok(value_type)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}{}=", indent_fn(indent), self.name)?;
        self.value.fmt(f)?;
        Ok(())
    }
    fn add_to_lookup(&self,compiler:&mut Compiler,symbols:&mut GlobalSymbols) {
        
    }
}
/*
 * Array node
 */
impl Compilable for ArrayNode {
    fn compile(&self, _compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        todo!()
    }

    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Array [", " ".repeat(indent))?;
        for element in &self.elements {
            element.fmt_with_indent(f, indent + 2)?;
        }
        writeln!(f, "{}]", " ".repeat(indent))
    }
    fn add_to_lookup(&self,compiler:&mut Compiler,symbols:&mut GlobalSymbols) {
        
    }
}
impl Compilable for FunctionCallNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        match self.call_type {
            CallType::Macro => {
                // HACK: temporarily remove the macro from the map so we can call `compile`.
                // Otherwise the borrow checker complains because `compile` needs
                // a mutable reference to `compiler`, while the macro is stored inside it.
                let mac = compiler.macros.macros.remove(&self.name).ok_or(
                    CompileError::UnknownMacro {
                        name: self.name.clone(),
                    },
                )?;
                let result = mac.compile(compiler, &self.args);
                compiler.macros.macros.insert(self.name.clone(), mac);
                result
            }
            CallType::Fn => {
                let old_fn = compiler.current_fn.clone();
                let called_function: CompileTimeFunctionForCheck =
                    compiler.context.get_fn(&self.name)?;
                compiler.current_fn = self.name.clone();
                if self.args.len() != called_function.args.len() {
                    return Err(CompileError::UnexpectedFunctionArguments {
                        name: self.name.clone(),
                        expected: called_function.args.len(),
                        found: self.args.len(),
                    });
                }
                compiler.context.enter_scope();
                for (called_arg, fnc_arg) in self.args.iter().zip(called_function.args.iter()) {
                    let called_args_type = called_arg.compile(compiler)?;

                    compiler.context.add_variable(
                        fnc_arg.name.clone(),
                        ComptimeVariable {
                            value_type: called_args_type.clone(),
                            is_const: false,
                            tag: format!("{}{}", fnc_arg.name.clone(), self.name.clone()),
                        },
                    )?;
                    let tag = compiler.context.get_variable(&fnc_arg.name).unwrap();
                    compiler.out.push(Instructions::SaveVar(tag.tag.clone()));
                    let final_fnc_type = CompileContext::get_type(&fnc_arg.argument_type)?;
                    if called_args_type != final_fnc_type {
                        return Err(TypeMismatch {
                            expected: final_fnc_type,
                            found: called_args_type,
                        });
                    }
                }
                for statement in called_function.body {
                    statement.compile(compiler)?;
                }
                compiler.context.exit_scope();
                compiler.current_fn = old_fn;
                Ok(Void)
            }
        }
    }
    fn fmt_with_indent(&self, _f: &mut Formatter<'_>, _indent: usize) -> fmt::Result {
        writeln!(_f, "{}{}(...)", indent_fn(_indent), self.name)
    }
    fn add_to_lookup(&self,compiler:&mut Compiler,symbols:&mut GlobalSymbols) {
       
        
    }
}

//FIXME:Add check for cyclic importing until imports are not macroed.
impl Compilable for ImportNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        

        return Ok(Void);
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        unimplemented!()
    }
    fn add_to_lookup(&self,compiler:&mut Compiler,symbols:&mut GlobalSymbols) {
        /*
         * Lexer
         */
        let mut main_lexer: Lexer = Lexer::new(
            fs::read_to_string(format!("src/{}", &self.module))
                .expect(format!("Cannot find module {}", &self.module).as_ref()),
        );
        let tokens: &Vec<Token> = match main_lexer.tokenize() {
            Err(e) => {
                println!("Error at {}:", &self.module);
                print!("{}", e);
                process::exit(-1);
            }
            Ok(tokens) => tokens,
        };
        /*
         * Parser
         */
        let mut main_parser: Parser = Parser::new(tokens.to_vec());
        let parsed_ast = main_parser.parse().unwrap_or_else(|e| {
            println!("Error at {}:", &self.module);
            println!("\x1b[1;31m{}\x1b[0m", e);
            process::exit(-2)
        });
        parsed_ast.add_to_lookup(compiler, symbols);
        
    }
}
