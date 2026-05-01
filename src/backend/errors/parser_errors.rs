use thiserror::Error;

use crate::backend::lexer::tokens::TokenKind;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Syntax error:expected {expected:?} but found {found}")]
    UnexpectedToken { found: String, expected: TokenKind },
    #[error("Syntax error:expected type but found {found}")]
    ExpectedType { found: String },
}
