use std::fmt;

use super::token::Span;

pub type ScannerResult<T> = Result<T, ScannerError>;

#[derive(Debug)]
pub enum ScannerErrorKind {
    UnknownToken { character: char },
    UnterminatedString,
}

impl fmt::Display for ScannerErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ScannerErrorKind::*;
        match &self {
            UnknownToken { character } => write!(f, "Unknow token: {character}"),
            UnterminatedString => write!(f, "Underminated string"),
        }
    }
}

pub struct ScannerError {
    kind: ScannerErrorKind,
    span: Span,
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}", self.span.to_location(), self.kind)
    }
}
// TODO: properly implement the Error trait https://doc.rust-lang.org/std/error/trait.Error.html
