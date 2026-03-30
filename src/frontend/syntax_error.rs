use super::TokenKind;
use super::token::Token;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum SyntaxError {
    // pass through error and diagnostics
    #[error(transparent)]
    Parser(#[from] ParserError),

    // pass through error and diagnostics
    #[error(transparent)]
    #[diagnostic(transparent)]
    Scanner(#[from] ScannerError),
}

#[derive(Error, Debug, Diagnostic)]
pub enum ScannerError {
    #[error("Unknown token")]
    UnknownToken {
        #[label("What is this?")]
        at: SourceSpan,
    },

    #[error("Unterminated string")]
    UnterminatedString {
        #[label("Missing terminating double quote")]
        at: SourceSpan,
    },
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Expected token: {message}")]
    ExpectedToken {
        token_type: TokenKind,
        message: &'static str,
    },

    #[error("Unexpected EOF: {message}")]
    UnexpectedEof { message: &'static str },

    #[error("Expected expression")]
    ExpectedExpression,

    #[error("Invalid number: {} at {}", token.lexeme, token.span.to_location())]
    InvalidNumber { token: Token },

    #[error("Invalid assignment to {token}")]
    InvalidAssignmentTarget { token: Token },
}
