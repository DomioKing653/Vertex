use thiserror::Error;
use toml::Value;
#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error(
        "Runtime error: Stack underflow at instruction {instr}! Something is wrong, this error is for debug builds of Vertex only."
    )]
    StackUnderFlow { instr: usize },
    #[error(
        "Runtime error: Missmatched types: expected {expceted} but found {found}! Something is wrong, this error is for debug builds of Vertex only."
    )]
    MissMatchedTypes { expceted: Value, found: Value },
}
