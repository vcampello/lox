use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("Runtime error")]
pub struct RuntimeError {
    pub kind: RuntimeKind,
    // FIXME: Expr and Stmts need to carry a span first for this to make sense
    // #[label("{kind}")]
    // pub at: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
pub enum RuntimeKind {
    #[error("Invalid operation")]
    InvalidOperation,

    #[error("Invalid arithmetic operation")]
    InvalidArithmeticOperation,

    #[error("Continue")]
    Continue,

    #[error("Break")]
    Break,

    #[error("Undefined variable '{name}'")]
    UndefinedVariable { name: String },
}
