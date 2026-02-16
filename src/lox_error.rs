// TODO: properly implement the Error trait https://doc.rust-lang.org/std/error/trait.Error.html
use crate::{
    backend::RuntimeError,
    frontend::{ParserError, SyntaxError},
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
