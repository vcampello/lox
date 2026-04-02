use crate::{
    backend::RuntimeError,
    frontend::{ParserError, ScannerError, SyntaxError},
};
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum LoxError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Syntax(#[from] SyntaxError),

    #[error("Runtime error")]
    #[diagnostic(transparent)]
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
