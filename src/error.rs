use thiserror::Error;

use crate::Identifier;

#[derive(Debug, Error)]
pub enum LoxError {
    #[error("ScannerError")]
    ScannerError(Vec<ScannerError>),
    #[error(transparent)]
    ParserError(#[from] ParserError),
    #[error(transparent)]
    RuntimeError(#[from] RuntimeError),
}

#[derive(Debug, Error)]
#[error("ScannerError [{line}]: {message}")]
pub struct ScannerError {
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Error)]
#[error("ParserError")]
pub struct ParserError;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum RuntimeError {
    UnboundVariable(#[from] UnboundVariable),
    TypeError(#[from] TypeError),
}

#[derive(Debug, Error)]
#[error("UboundVariable: {0}")]
pub struct UnboundVariable(pub Identifier);

#[derive(Debug, Error)]
#[error("TypeError")]
pub struct TypeError;