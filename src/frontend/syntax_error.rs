use super::token::{Token, TokenType};
use thiserror::Error;

pub type ScannerResult<T> = Result<T, SyntaxError>;

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Parser error: {0}")]
    Parser(#[from] ParserError),

    #[error("Scanner error: {0}")]
    Scanner(#[from] ScannerError),
}

#[derive(Error, Debug)]
pub enum ScannerError {
    // TODO: add scanner errors
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
