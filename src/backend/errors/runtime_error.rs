use thiserror::Error;
#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error(
        "Runtime error: Stack underflow at instruction {instr}! Something is wrong, this error is for debug builds of Vertex only."
    )]
    StackUnderFlow { instr: usize },
}
