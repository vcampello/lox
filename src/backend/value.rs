use std::fmt;

use crate::ast::FunctionStmt;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Bool(bool),
    Number(f64),
    String(String),
    Function(FunctionStmt),
    Nil,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match &self {
            Value::Nil => false,
            Value::Bool(v) => *v,

            // lox follows the ruby rules: everything aside from nil is true
            _ => true,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(v) => write!(f, "{v}"),
            Self::Number(v) => write!(f, "{v}"),
            Self::Bool(v) => write!(f, "{v}"),
            Self::Nil => write!(f, "nil"),
            Self::Function(v) => write!(f, "<funtion {}>", v.name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nil_is_false() {
        assert!(!Value::Nil.is_truthy());
    }

    #[test]
    fn true_is_true() {
        // I know this test seems redundant...
        assert!(Value::Bool(true).is_truthy());
    }

    #[test]
    fn false_is_false() {
        // I know this test seems redundant...
        assert!(!Value::Bool(false).is_truthy());
    }

    #[test]
    fn zero_is_true() {
        assert!(Value::Number(0.0).is_truthy());
    }

    #[test]
    fn one_is_true() {
        assert!(Value::Number(1.0).is_truthy());
    }

    #[test]
    fn empty_string_is_true() {
        assert!(Value::String(String::new()).is_truthy());
    }

    #[test]
    fn string_is_true() {
        assert!(Value::String("abc".to_string()).is_truthy());
    }
}
