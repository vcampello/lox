use super::TokenKind;
use crate::common::Span;
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, span: Span) -> Self {
        Self { kind, lexeme, span }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " {}| {}", self.span.to_location(), self.kind)
    }
}
