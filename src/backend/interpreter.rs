use std::fmt;

use crate::{
    ast::{Expr, Stmt},
    backend::{Env, RuntimeError, RuntimeErrorKind},
    frontend::TokenType,
};

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

pub type InterpreterResult<T> = Result<T, RuntimeError>;

#[derive(Debug, Default)]
pub struct Interpreter {
    env: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { env: Env::new() }
    }

    pub fn interpret(&mut self, stmts: &[Stmt]) -> InterpreterResult<()> {
        for stmt in stmts.iter() {
            match stmt {
                Stmt::Print(expr) => {
                    let result = self.evaluate(expr)?;
                    println!("{result}");
                }
                Stmt::Expression(expr) => _ = self.evaluate(expr)?,
                Stmt::Var { name, initializer } => {
                    let value = match initializer {
                        Some(expr) => self.evaluate(expr)?,
                        None => Value::Nil,
                    };
                    self.env.define(&name.lexeme, &value);
                }
                Stmt::Block(stmts) => {
                    self.env.begin_scope();
                    self.interpret(stmts)?;
                    self.env.end_scope();
                }
            };
        }

        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> InterpreterResult<Value> {
        match expr {
            Expr::BoolLiteral(v) => Ok(Value::Bool(*v)),
            Expr::StringLiteral(v) => Ok(Value::String(v.clone())),
            Expr::NumberLiteral(v) => Ok(Value::Number(*v)),
            Expr::Nil => Ok(Value::Nil),
            Expr::Grouping(expr) => self.evaluate(expr),

            Expr::Unary { operator, right } => {
                let right_result = self.evaluate(right)?;

                match (&operator.token_type, right_result) {
                    (TokenType::Minus, Value::Number(v)) => Ok(Value::Number(-v)),
                    (TokenType::Bang, v) => {
                        let is_true = Interpreter::is_truthy(&v);
                        Ok(Value::Bool(!is_true))
                    }
                    _ => Err(RuntimeError {
                        kind: RuntimeErrorKind::InvalidOperation,
                    }),
                }
            }

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let right_resut = self.evaluate(right)?;
                let left_result = self.evaluate(left)?;

                match (&operator.token_type, left_result, right_resut) {
                    // arithmetic
                    (TokenType::Slash, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Number(l / r))
                    }
                    (TokenType::Star, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Number(l * r))
                    }
                    (TokenType::Minus, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Number(l - r))
                    }
                    (TokenType::Plus, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Number(l + r))
                    }

                    // string concatenation
                    (TokenType::Plus, Value::String(l), r) => Ok(Value::String(l + &r.to_string())),
                    (TokenType::Plus, l, Value::String(r)) => Ok(Value::String(l.to_string() + &r)),

                    // comparison
                    (TokenType::Greater, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Bool(l > r))
                    }
                    (TokenType::GreaterEqual, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Bool(l >= r))
                    }
                    (TokenType::Less, Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l < r)),
                    (TokenType::LessEqual, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Bool(l <= r))
                    }

                    // equality - number
                    (TokenType::EqualEqual, l, r) => Ok(Value::Bool(l == r)),
                    (TokenType::BangEqual, l, r) => Ok(Value::Bool(l != r)),

                    _ => Err(RuntimeError {
                        kind: RuntimeErrorKind::InvalidOperation,
                    }),
                }
            }

            Expr::Variable { name } => {
                let v = self
                    .env
                    .get(&name.lexeme)
                    // FIXME: this error handling is a mess but let's leave it here until I get to
                    // the rewrite
                    .map_err(|_| RuntimeError {
                        kind: RuntimeErrorKind::UndefinedVariable {
                            name: name.lexeme.to_string(),
                        },
                    })?;

                Ok(v.clone())
            }

            Expr::Assignment { name, value } => {
                let result = self.evaluate(value)?;
                self.env
                    .assign(&name.lexeme, &result)
                    .map_err(|_| RuntimeError {
                        // FIXME: this error handling is a mess but let's leave it here until I get to
                        // the rewrite
                        kind: RuntimeErrorKind::UndefinedVariable {
                            name: name.lexeme.to_string(),
                        },
                    })?;

                Ok(result)
            }

            // FIXME: remove once conditionals are added
            _ => Err(RuntimeError {
                kind: RuntimeErrorKind::Unimplemented,
            }),
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
