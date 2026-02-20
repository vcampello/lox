use super::*;
use crate::{
    ast::{Expr, Stmt},
    frontend::TokenType,
};
use std::slice;

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
                Stmt::Conditional {
                    condition,
                    when_true,
                    when_false,
                } => {
                    if self.evaluate(condition)?.is_truthy() {
                        // else cute if branch
                        self.interpret(slice::from_ref(when_true))?;
                    } else if let Some(stmt) = when_false {
                        // execute else branch if defined
                        self.interpret(slice::from_ref(stmt))?;
                    }
                }
                Stmt::While { condition, body } => {
                    while self.evaluate(condition)?.is_truthy() {
                        self.interpret(slice::from_ref(body))?;
                    }
                }
                Stmt::For {
                    initializer,
                    condition,
                    increment,
                    body,
                } => {
                    // capture for loop initializer in a new scope
                    self.env.begin_scope();

                    if let Some(initializer) = initializer {
                        self.interpret(slice::from_ref(initializer))?;
                    }

                    while match condition {
                        Some(expr) => self.evaluate(expr)?.is_truthy(),
                        None => true,
                    } {
                        self.interpret(slice::from_ref(body))?;

                        if let Some(increment) = increment {
                            self.evaluate(increment)?;
                        }
                    }
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
                    (TokenType::Bang, v) => Ok(Value::Bool(!v.is_truthy())),
                    _ => Err(RuntimeError::InvalidOperation),
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

                    _ => Err(RuntimeError::InvalidOperation),
                }
            }

            Expr::Logical {
                left,
                operator,
                right,
            } => match operator.token_type {
                TokenType::And => {
                    let left_result = self.evaluate(left)?;
                    match left_result.is_truthy() {
                        // short circuit
                        false => Ok(left_result),
                        // keep chaining so long as it's true
                        true => self.evaluate(right),
                    }
                }
                TokenType::Or => {
                    let left_result = self.evaluate(left)?;
                    match left_result.is_truthy() {
                        // short circuit
                        true => Ok(left_result),
                        // keep chaining
                        false => self.evaluate(right),
                    }
                }
                _ => Err(RuntimeError::InvalidOperation),
            },

            Expr::Variable { name } => Ok(self.env.get(&name.lexeme)?.clone()),

            Expr::Assignment { name, value } => {
                let result = self.evaluate(value)?;
                self.env.assign(&name.lexeme, &result)?;
                Ok(result)
            }
        }
    }
}
