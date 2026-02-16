use crate::ast::Expr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Invalid operation")]
    InvalidOperation,

    #[error("Invalid arithmetic operation")]
    InvalidArithmeticOperation,

    // FIXME: implement fmt::Display for Expr
    #[error("Unimplemented: todo!")]
    Unimplemented { expr: Expr },

    #[error("Environment error: {0}")]
    Environment(#[from] EnvironmentError),
}

#[derive(Error, Debug)]
pub enum EnvironmentError {
    #[error("Undefined variable: {name}")]
    UndefinedVariable { name: String },
}
