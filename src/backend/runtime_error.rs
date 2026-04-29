// Allow unused assignments - required by miette::Diagnostic derive macro
#![allow(unused_assignments)]

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::common::Span;

#[derive(Error, Debug, Diagnostic)]
#[error("{kind}")]
pub struct RuntimeError {
    pub kind: RuntimeErrorKind,

    #[label("{kind}")]
    pub span: SourceSpan,
}

impl RuntimeError {
    pub fn invalid_op(operation: String, span: Span) -> Self {
        Self {
            kind: RuntimeErrorKind::InvalidOperation { operation },
            span: span.into(),
        }
    }

    pub fn undefined_var(name: String, span: Span) -> Self {
        Self {
            kind: RuntimeErrorKind::UndefinedVariable { name },
            span: span.into(),
        }
    }

    pub fn continue_signal(span: Span) -> Self {
        Self {
            kind: RuntimeErrorKind::Continue,
            span: span.into(),
        }
    }

    pub fn break_signal(span: Span) -> Self {
        Self {
            kind: RuntimeErrorKind::Break,
            span: span.into(),
        }
    }

    pub fn not_callable(span: Span) -> Self {
        Self {
            span: span.into(),
            kind: RuntimeErrorKind::NotCallable,
        }
    }

    pub fn incorrect_arity(expected: usize, received: usize, span: Span) -> Self {
        Self {
            span: span.into(),
            kind: RuntimeErrorKind::IncorrectArity { expected, received },
        }
    }
}

#[derive(Error, Debug, Diagnostic)]
pub enum RuntimeErrorKind {
    #[error("Invalid {operation} operation")]
    InvalidOperation { operation: String },

    #[error("Undefined variable '{name}'")]
    UndefinedVariable { name: String },

    #[error("Continue")]
    Continue,

    #[error("Break")]
    Break,

    #[error("Not callable")]
    NotCallable,

    #[error("Incorrect arity. Expected {expected} arguments but received {received}")]
    IncorrectArity { expected: usize, received: usize },
}
