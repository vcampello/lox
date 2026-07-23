// Allow unused assignments - required by miette::Diagnostic derive macro
#![allow(unused_assignments)]

use crate::{ast::TokenKind, common::Span};
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Clone, Debug, Diagnostic)]
#[error("syntax error")]
pub enum SyntaxError {
    // pass through diagnostics
    #[diagnostic(transparent)]
    Scanner(#[from] ScannerError),

    // pass through diagnostics
    #[diagnostic(transparent)]
    Parser(#[from] ParserError),
}

#[derive(Error, Clone, Debug, Diagnostic)]
#[error("{kind}")]
pub struct ScannerError {
    pub kind: ScannerErrorKind,

    #[label("{kind}")]
    pub span: SourceSpan,
}

impl ScannerError {
    pub fn unterminated_string(span: Span) -> Self {
        Self {
            span: span.into(),
            kind: ScannerErrorKind::UnterminatedString,
        }
    }

    pub fn unexpected_character(ch: char, span: Span) -> Self {
        Self {
            span: span.into(),
            kind: ScannerErrorKind::UnexpectedCharacter(ch),
        }
    }
}

#[derive(Error, Debug, Clone)]
pub enum ScannerErrorKind {
    #[error("unterminated string")]
    UnterminatedString,

    #[error("unexpected character {0}")]
    UnexpectedCharacter(char),
}

#[derive(Error, Clone, Debug, Diagnostic)]
#[error("{kind}")]
pub struct ParserError {
    pub kind: ParserErrorKind,

    #[label("{kind}")]
    pub span: SourceSpan,
}

impl ParserError {
    pub fn expected_token(
        message: String,
        expected: TokenKind,
        found: TokenKind,
        span: Span,
    ) -> Self {
        Self {
            span: span.into(),
            kind: ParserErrorKind::ExpectedToken {
                expected,
                found,
                message,
            },
        }
    }

    pub fn unexpected_eof(message: String, span: Span) -> Self {
        Self {
            span: span.into(),
            kind: ParserErrorKind::UnexpectedEof { message },
        }
    }

    pub fn expected_expression(span: Span) -> Self {
        Self {
            span: span.into(),
            kind: ParserErrorKind::ExpectedExpression,
        }
    }

    pub fn invalid_number(lexeme: String, span: Span) -> Self {
        Self {
            span: span.into(),
            kind: ParserErrorKind::InvalidNumber { lexeme },
        }
    }

    pub fn invalid_assignment_target(token_kind: TokenKind, span: Span) -> Self {
        Self {
            span: span.into(),
            kind: ParserErrorKind::InvalidAssignmentTarget { token_kind },
        }
    }

    pub fn invalid_operator(operation: &'static str, token_kind: TokenKind, span: Span) -> Self {
        Self {
            span: span.into(),
            kind: ParserErrorKind::InvalidOperator {
                operation,
                token_kind,
            },
        }
    }

    pub fn too_many_arguments(max_args: usize, span: Span) -> Self {
        Self {
            span: span.into(),
            kind: ParserErrorKind::TooManyArguments { max_args },
        }
    }
}

#[derive(Error, Clone, Debug)]
pub enum ParserErrorKind {
    #[error("expected token {expected} but found {found}: {message}")]
    ExpectedToken {
        expected: TokenKind,
        found: TokenKind,
        message: String,
    },

    #[error("unexpected EOF: {message}")]
    UnexpectedEof { message: String },

    #[error("expected expression")]
    ExpectedExpression,

    #[error("invalid number: {lexeme} ")]
    InvalidNumber { lexeme: String },

    #[error("invalid assignment to {token_kind}")]
    InvalidAssignmentTarget { token_kind: TokenKind },

    #[error("invalid {operation} operator: {token_kind}")]
    InvalidOperator {
        operation: &'static str,
        token_kind: TokenKind,
    },

    #[error("exceeded {max_args} arguments")]
    TooManyArguments { max_args: usize },
}
