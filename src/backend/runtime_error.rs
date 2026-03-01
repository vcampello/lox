use crate::ast::Expr;
use miette::Diagnostic;
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

    // pass through error and diagnostics
    #[error(transparent)]
    #[diagnostic(transparent)]
    Environment(#[from] EnvironmentError),
}

#[derive(Error, Debug, Diagnostic)]
pub enum EnvironmentError {
    #[error("Undefined variable '{name}'")]
    UndefinedVariable { name: String },
}
