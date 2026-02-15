use super::token::{Token, TokenType};

pub type ScannerResult<T> = Result<T, SyntaxError>;

#[derive(Debug)]
pub struct SyntaxError {
    pub kind: SyntaxErrorKind,
}

#[derive(Debug)]
pub enum SyntaxErrorKind {
    Parser(ParserError),
}

#[derive(Debug)]
pub struct ParserError {
    pub kind: ParserErrorKind,
}

#[derive(Debug)]
pub enum ParserErrorKind {
    ExpectedToken(TokenType),
    ExpectedExpression,
    InvalidNumber(String),
    InvalidAssignmentTarget(Token),
}

// -----------------------------------------------------------------------------
// automatic conversion
// -----------------------------------------------------------------------------
impl From<ParserError> for SyntaxError {
    fn from(value: ParserError) -> Self {
        Self {
            kind: SyntaxErrorKind::Parser(value),
        }
    }
}
