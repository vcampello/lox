// Allow unused assignments - required by miette::Diagnostic derive macro
#![allow(unused_assignments)]

use crate::{ast::TokenKind, common::Span};
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("Syntax error")]
pub enum SyntaxError {
    // pass through diagnostics
    #[diagnostic(transparent)]
    Scanner(#[from] ScannerError),

    // pass through diagnostics
    #[diagnostic(transparent)]
    Parser(#[from] ParserError),
}

#[derive(Error, Debug, Diagnostic)]
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
    #[error("Unterminated string")]
    UnterminatedString,

    #[error("Unexpected character {0}")]
    UnexpectedCharacter(char),
}

#[derive(Error, Debug, Diagnostic)]
#[error("{kind}")]
pub struct ParserError {
    pub kind: ParserErrorKind,

    #[label("{kind}")]
    pub span: SourceSpan,
}

impl ParserError {
    pub fn expected_token(message: &'static str, token_kind: TokenKind, span: Span) -> Self {
        Self {
            span: span.into(),
            kind: ParserErrorKind::ExpectedToken {
                token_kind,
                message,
            },
        }
    }

    pub fn unexpected_eof(message: &'static str, span: Span) -> Self {
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
}

#[derive(Error, Debug)]
pub enum ParserErrorKind {
    #[error("Expected token {token_kind}: {message}")]
    ExpectedToken {
        token_kind: TokenKind,
        message: &'static str,
    },

    #[error("Unexpected EOF: {message}")]
    UnexpectedEof { message: &'static str },

    #[error("Expected expression")]
    ExpectedExpression,

    #[error("Invalid number: {lexeme} ")]
    InvalidNumber { lexeme: String },

    #[error("Invalid assignment to {token_kind}")]
    InvalidAssignmentTarget { token_kind: TokenKind },

    #[error("Invalid {operation} operator: {token_kind}")]
    InvalidOperator {
        operation: &'static str,
        token_kind: TokenKind,
    },
}
