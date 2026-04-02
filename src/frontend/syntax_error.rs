// Allow unused assignments - required by miette::Diagnostic derive macro
#![allow(unused_assignments)]

use crate::ast::TokenKind;
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
#[error("Scanner error")]
pub struct ScannerError {
    pub kind: ScannerErrorKind,

    #[label("{kind}")]
    pub span: SourceSpan,
}

#[derive(Error, Debug, Clone)]
pub enum ScannerErrorKind {
    #[error("Unterminated string")]
    UnterminatedString,

    #[error("Unexpected character {0}")]
    UnexpectedCharacter(char),
}

// TODO: add factory methods
#[derive(Error, Debug, Diagnostic)]
#[error("Parser error")]
pub struct ParserError {
    pub kind: ParserErrorKind,

    #[label("{kind}")]
    pub span: SourceSpan,
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
