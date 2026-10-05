use crate::backend::lexer::tokens::Span;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum LexerErrorKind {
    #[error("Unknown token: {wrong_token}")]
    UnknownToken { wrong_token: String },
    #[error("Cannot have more dots in a number")]
    MoreDotInANumber,
    #[error("Unterminated string:{text}")]
    UnterminatedString { text: String },
    #[error("Cannot parse empty file")]
    EmptyFile,
}
#[derive(Error, Debug)]
// #[error("ln:{line},ch:{char} -> {err}")]
#[error("{span:?} -> {err}")]
pub struct LexerError {
    pub err: LexerErrorKind,
    // pub line: usize,
    // pub char: usize,
    // Addition:
    pub span: Span,
}
