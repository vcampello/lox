use super::token::{Token, TokenType};

pub type ScannerResult<T> = Result<T, SyntaxError>;

#[derive(Debug)]
pub enum SyntaxError {
    Parser(ParserError),
    Scanner(ScannerError),
}

#[derive(Debug)]
pub enum ScannerError {
    // TODO: add scanner errors
}

#[derive(Debug)]
pub enum ParserError {
    ExpectedToken { token_type: TokenType },
    ExpectedExpression,
    InvalidNumber { lexme: String },
    InvalidAssignmentTarget { token: Token },
}

// -----------------------------------------------------------------------------
// automatic conversion
// -----------------------------------------------------------------------------
impl From<ParserError> for SyntaxError {
    fn from(value: ParserError) -> Self {
        Self::Parser(value)
    }
}

impl From<ScannerError> for SyntaxError {
    fn from(value: ScannerError) -> Self {
        Self::Scanner(value)
    }
}
