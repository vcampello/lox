use crate::ast::Expr;

#[derive(Debug)]
pub enum RuntimeError {
    Interpreter(InterpreterError),
    Environment(EnvironmentError),
}

#[derive(Debug)]
pub enum InterpreterError {
    InvalidOperation,
    InvalidArithmeticOperation,
    Unimplemented { expr: Expr },
    UndefinedVariable { name: String },
}

#[derive(Debug)]
pub enum EnvironmentError {
    UndefinedVariable { name: String },
}

// -----------------------------------------------------------------------------
// automatic conversion
// -----------------------------------------------------------------------------
impl From<InterpreterError> for RuntimeError {
    fn from(value: InterpreterError) -> Self {
        Self::Interpreter(value)
    }
}

impl From<EnvironmentError> for RuntimeError {
    fn from(value: EnvironmentError) -> Self {
        Self::Environment(value)
    }
}
