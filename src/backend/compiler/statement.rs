use crate::backend::ast::{
    functions::function_nodes::FunctionDefineNode,
    nodes::{FunctionCallNode, VariableAccessNode, VariableAssignNode, VariableDefineNode},
    statements::{if_statement::IfStatement, while_statement::WhileStatement},
};

pub enum Statements {
    If(IfStatement),
    While(WhileStatement),
    FunctionCall(FunctionCallNode),
    VariableAcess(VariableAccessNode),
    VariableAssign(VariableAssignNode),
    VariableDefine(VariableDefineNode),
    FunctionDeclaration(FunctionDefineNode),
}
