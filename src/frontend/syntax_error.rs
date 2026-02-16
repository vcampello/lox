use crate::frontend::Span;

use super::token::{Token, TokenType};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Parser error: {0}")]
    Parser(#[from] ParserError),

    #[error("Scanner error: {0}")]
    Scanner(#[from] ScannerError),
}

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Unknown token: {token} at {}", span.to_location())]
    UnknownToken { token: char, span: Span },

    #[error("Unterminated string at {}", span.to_location())]
    UnterminatedString { span: Span },
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Expected token: {token_type}")]
    ExpectedToken { token_type: TokenType },

    #[error("Expected expression")]
    ExpectedExpression,

    #[error("Invalid number: {lexeme}")]
    InvalidNumber { lexeme: String },

    #[error("Invalid assignment to {token}")]
    InvalidAssignmentTarget { token: Token },
}
