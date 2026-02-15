use std::fmt;

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Debug)]
pub struct RuntimeError {
    pub kind: RuntimeErrorKind,
}

#[derive(Debug)]
pub enum RuntimeErrorKind {
    // TODO: add meaningful data
    InvalidOperation,
    InvalidArithmeticOperation,
    Unimplemented,
    UndefinedVariable { name: String },
}

impl fmt::Display for RuntimeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeErrorKind::InvalidOperation => write!(f, "InvalidOperation"),
            RuntimeErrorKind::InvalidArithmeticOperation => write!(f, "InvalidArithmeticOperation"),
            RuntimeErrorKind::UndefinedVariable { name } => write!(f, "UndefinedVariable({name})"),
            RuntimeErrorKind::Unimplemented => write!(f, "Unimplemented"),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RuntimeError: {}", self.kind)
    }
}
