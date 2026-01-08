use std::fmt;

use crate::{
    ast::expression::Expr,
    token::{Token, TokenType},
};

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Number(f64),
    String(String),
    Nil,
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

#[derive(Clone, Debug)]
pub enum RuntimeError {
    // TOOD: flesh it out
    Something,
}

pub type InterpreterResult = Result<Value, RuntimeError>;

pub struct Interpreter {}

impl Interpreter {
    pub fn visit(expr: &Expr) -> InterpreterResult {
        match expr {
            Expr::BoolLiteral(v) => Ok(Value::Bool(*v)),
            Expr::StringLiteral(v) => Ok(Value::String(v.clone())),
            Expr::NumberLiteral(v) => Ok(Value::Number(*v)),
            Expr::Nil => Ok(Value::Nil),
            Expr::Grouping(expr) => Interpreter::visit(expr),

            Expr::Unary { operator, right } => {
                let right_result = Interpreter::visit(right)?;

                match (&operator.token_type, right_result) {
                    (TokenType::Minus, Value::Number(v)) => Ok(Value::Number(-v)),
                    (TokenType::Bang, v) => {
                        let is_true = Interpreter::is_truthy(&v);
                        Ok(Value::Bool(!is_true))
                    }
                    _ => Ok(Value::Bool(false)),
                }
            }

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let right_resut = Interpreter::visit(right)?;
                let left_resut = Interpreter::visit(left)?;

                match (&operator.token_type, left_resut, right_resut) {
                    (TokenType::Slash, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Number(a / b))
                    }
                    (TokenType::Star, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Number(a * b))
                    }
                    (TokenType::Minus, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Number(a - b))
                    }
                    (TokenType::Plus, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Number(a + b))
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }

    /// Lox follows Ruby’s simple rule: false and nil are falsey, and everything else is truthy.
    pub fn is_truthy(value: &Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Bool(v) => *v,
            _ => true,
        }
    }
}
