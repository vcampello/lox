use std::{fmt, ops};

use crate::{ast::expression::Expr, token::TokenType};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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

impl ops::Add<Value> for Value {
    type Output = Value;
    fn add(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            // maths!
            (Value::Number(l), Value::Number(r)) => Value::Number(l + r),

            // strings
            (Value::String(l), r) => Value::String(l + &r.to_string()),
            (l, Value::String(r)) => Value::String(l.to_string() + &r),

            // Adding anything with nil should be nil
            (Value::Nil, _) => Value::Nil,
            (_, Value::Nil) => Value::Nil,

            // Anything else is probably NaN
            _ => Value::Number(f64::NAN),
        }
    }
}

#[derive(Clone, Debug)]
pub enum RuntimeError {
    // TODO: this should probably be a combination of top level errors with different payloads.
    // e.g. InvalidExpression(expr)
    // e.g. InvalidOperation(...)
    InvalidOperation,
    InvalidArithmeticOperation,
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

                // REVIEW: would it be more readable to have a nested match?
                match (&operator.token_type, left_resut, right_resut) {
                    // arithmetic
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

                    // string concatenation (uses a custom ops::Add trait)
                    (TokenType::Plus, a, b) => Ok(Value::String((a + b).to_string())),

                    // comparison
                    (TokenType::Greater, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Bool(a > b))
                    }
                    (TokenType::GreaterEqual, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Bool(a >= b))
                    }
                    (TokenType::Less, Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
                    (TokenType::LessEqual, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Bool(a <= b))
                    }

                    // equality - number
                    (TokenType::EqualEqual, a, b) => Ok(Value::Bool(a == b)),
                    (TokenType::BangEqual, a, b) => Ok(Value::Bool(a != b)),

                    _ => Err(RuntimeError::InvalidOperation),
                }
            }
            _ => todo!(),
        }
    }

    // REVIEW: should this be part of the Interpreter or Value?
    /// Lox follows Ruby’s simple rule: false and nil are falsey, and everything else is truthy.
    pub fn is_truthy(value: &Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Bool(v) => *v,
            _ => true,
        }
    }
}
