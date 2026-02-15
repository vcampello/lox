// TODO: properly implement the Error trait https://doc.rust-lang.org/std/error/trait.Error.html
use crate::{
    backend::{EnvironmentError, InterpreterError, RuntimeError},
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

impl From<ParserError> for LoxError {
    fn from(value: ParserError) -> Self {
        Self::Syntax(value.into())
    }
}

impl From<EnvironmentError> for LoxError {
    fn from(value: EnvironmentError) -> Self {
        Self::Runtime(value.into())
    }
}

impl From<InterpreterError> for LoxError {
    fn from(value: InterpreterError) -> Self {
        Self::Runtime(value.into())
    }
}
