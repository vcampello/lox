use crate::{
    backend::RuntimeError,
    frontend::{ParserError, ScannerError, SyntaxError},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoxError {
    #[error("Syntax error: {0}")]
    Syntax(#[from] SyntaxError),

    #[error("Runtime error: {0}")]
    Runtime(#[from] RuntimeError),
}

// -----------------------------------------------------------------------------
// automatic conversion
// -----------------------------------------------------------------------------
impl From<ParserError> for LoxError {
    fn from(value: ParserError) -> Self {
        Self::Syntax(value.into())
    }
}

impl From<ScannerError> for LoxError {
    fn from(value: ScannerError) -> Self {
        Self::Syntax(value.into())
    }
}
