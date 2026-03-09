use crate::backend::compiler::byte_code::Compilable;
use crate::backend::lexer::tokens::TokenKind;
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub enum CallType {
    Macro,
    Fn,
}

#[derive(Clone)]
pub struct ProgramNode {
    pub program_nodes: Vec<Box<dyn Compilable>>,
}

impl ProgramNode {
    pub fn new() -> Self {
        Self {
            program_nodes: Vec::new(),
        }
    }
}

impl fmt::Debug for ProgramNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
impl Default for ProgramNode {
    fn default() -> Self {
        Self::new()
    }
}
/*
Binary Operation Node
*/
#[derive(Clone)]
pub struct BinaryOpNode {
    pub left: Box<dyn Compilable>,
    pub right: Box<dyn Compilable>,
    pub op_tok: TokenKind,
}

impl fmt::Debug for BinaryOpNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
/*
 * Unary expresion node
 */
#[derive(Clone)]
pub struct PrefixExpressionNode {
    pub prefix: TokenKind,
    pub value: Box<dyn Compilable>,
}

/*
Number Node
*/
#[derive(Clone)]
pub struct NumberNode {
    pub number: i64,
}

impl fmt::Debug for NumberNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
/*
Float node
*/
#[derive(Clone)]
pub struct FloatNode {
    pub number: f32,
}

impl fmt::Debug for FloatNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
/*
 * String node
 */

#[derive(Clone)]
pub struct StringNode {
    pub value: String,
}
impl Debug for StringNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
/*
 * Bool node
 */
#[derive(Clone)]
pub struct BoolNode {
    pub value: TokenKind,
}
impl Debug for BoolNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
/*
 * Array node
 */
#[derive(Clone)]
pub struct ArrayNode {
    pub elements: Vec<Box<dyn Compilable>>,
}
impl Debug for ArrayNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
/*
Variable Access
*/

#[derive(Clone)]
pub struct VariableAccessNode {
    pub variable_name: String,
}

impl fmt::Debug for VariableAccessNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
/*
Variable Define
*/
#[derive(Clone)]
pub struct VariableDefineNode {
    pub var_name: String,
    pub value_type: Option<String>,
    pub value: Option<Box<dyn Compilable>>,
    pub is_const: bool,
    pub is_public: bool,
}
impl Debug for VariableDefineNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
/*
Variable assign node
*/

#[derive(Clone)]
pub struct VariableAssignNode {
    pub name: String,
    pub value: Box<dyn Compilable>,
}

impl Debug for VariableAssignNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

/*
FunctionCallNode
*/

#[derive(Clone)]
pub struct FunctionCallNode {
    pub args: Vec<Box<dyn Compilable>>,
    pub name: String,
    pub call_type: CallType,
}

impl Debug for FunctionCallNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

#[derive(Clone)]
pub struct ImportNode {
    pub module: String,
}

impl Debug for ImportNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
