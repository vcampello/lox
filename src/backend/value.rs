use std::fmt;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Bool(bool),
    Number(f64),
    String(String),
    Nil,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match &self {
            Value::Nil => false,
            Value::Bool(v) => *v,
            Value::Number(v) => *v > 0.0,
            Value::String(v) => !v.is_empty(),
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
        }
    }
}
