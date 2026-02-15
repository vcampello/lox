// TODO: properly implement the Error trait https://doc.rust-lang.org/std/error/trait.Error.html
use crate::{
    backend::RuntimeError,
    frontend::{ParserError, SyntaxError},
};

#[derive(Debug)]
pub enum LoxError {
    Syntax(SyntaxError),
    Runtime(RuntimeError),
}

// -----------------------------------------------------------------------------
// automatic conversion
// -----------------------------------------------------------------------------
impl From<SyntaxError> for LoxError {
    fn from(value: SyntaxError) -> Self {
        Self::Syntax(value)
    }
}

impl From<RuntimeError> for LoxError {
    fn from(value: RuntimeError) -> Self {
        Self::Runtime(value)
    }
}

// NOTE: it may be better to implement package specific functions
// to convert errors from lower -> upper domains
impl From<ParserError> for LoxError {
    fn from(value: ParserError) -> Self {
        Self::Syntax(value.into())
    }
}
