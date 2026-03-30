// Allow unused assignments - required by miette::Diagnostic derive macro
#![allow(unused_assignments)]

use super::TokenKind;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum SyntaxError {
    // pass through error and diagnostics
    #[error(transparent)]
    #[diagnostic(transparent)]
    Scanner(#[from] ScannerError),

    // pass through error and diagnostics
    #[error(transparent)]
    #[diagnostic(transparent)]
    Parser(#[from] ParserError),
}

#[derive(Error, Debug, Diagnostic)]
#[error("Scanner error")]
pub struct ScannerError {
    pub kind: ScannerErrorKind,

    #[label("{kind}")]
    pub at: SourceSpan,
}

#[derive(Error, Debug, Clone)]
pub enum ScannerErrorKind {
    #[error("unterminated string")]
    UnterminatedString,
    #[error("unexpected character '{0}'")]
    UnexpectedCharacter(char),
}

#[derive(Error, Debug, Diagnostic)]
#[error("Parser error")]
pub struct ParserError {
    pub kind: ParserErrorKind,

    #[label("{kind}")]
    pub at: SourceSpan,
}

#[derive(Error, Debug)]
pub enum ParserErrorKind {
    #[error("Expected token {token_type}: {message}")]
    ExpectedToken {
        token_type: TokenKind,
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
}
