use crate::ast::Expr;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum RuntimeError {
    #[error("Invalid operation")]
    InvalidOperation,

    #[error("Invalid arithmetic operation")]
    InvalidArithmeticOperation,

    #[error("Unimplemented expression: {expr}")]
    Unimplemented { expr: Expr },

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
