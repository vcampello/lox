use crate::{
    backend::RuntimeError,
    frontend::{ParserError, ScannerError, SyntaxError},
};
use miette::Diagnostic;
use thiserror::Error;

#[derive(Clone, Error, Debug, Diagnostic)]
pub enum LoxError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Syntax(#[from] SyntaxError),

    #[error("runtime error")]
    #[diagnostic(transparent)]
    Runtime(#[from] RuntimeError),
}

impl LoxError {
    pub fn flatten(&self) -> String {
        format!(
            "{}: {}",
            self,
            match self {
                LoxError::Runtime(e) => e.to_string(),
                LoxError::Syntax(e) => e.to_string(),
            }
        )
    }
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
