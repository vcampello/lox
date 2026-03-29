use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum RuntimeError {
    #[error("Invalid operation")]
    InvalidOperation,

    #[error("Invalid arithmetic operation")]
    InvalidArithmeticOperation,

    #[error("Continue")]
    Continue,

    #[error("Break")]
    Break,

    #[error("Undefined variable '{name}'")]
    UndefinedVariable {
        name: String,
        #[label("Is '{name}' actually defined?")]
        at: SourceSpan,
    },
}
