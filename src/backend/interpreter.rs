use super::*;
use crate::{
    ast::{Expr, ExprVisitor, Stmt, StmtVisitor, walk_expr, walk_stmt},
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
            walk_stmt(stmt, self)?;
        }

        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> InterpreterResult<Value> {
        walk_expr(expr, self)
    }
}

impl StmtVisitor for Interpreter {
    type Output = InterpreterResult<()>;

    fn visit_block(&mut self, stmts: &[Stmt]) -> Self::Output {
        // new scope for the upcoming block
        self.env.begin_scope();

        // the scope needs to be dropped regardless of the result
        let result = self.interpret(stmts);
        self.env.end_scope();

        result
    }

    fn visit_expression(&mut self, expr: &Expr) -> Self::Output {
        self.evaluate(expr)?;
        Ok(())
    }

    fn visit_print(&mut self, expr: &Expr) -> Self::Output {
        let result = self.evaluate(expr)?;
        println!("{result}");
        Ok(())
    }

    fn visit_variable(
        &mut self,
        name: &crate::frontend::Token,
        initializer: &Option<Expr>,
    ) -> Self::Output {
        let value = match initializer {
            Some(expr) => self.evaluate(expr)?,
            None => Value::Nil,
        };
        self.env.define(&name.lexeme, &value);
        Ok(())
    }

    fn visit_conditional(
        &mut self,
        condition: &Expr,
        when_true: &Stmt,
        when_false: &Option<Box<Stmt>>,
    ) -> Self::Output {
        if self.evaluate(condition)?.is_truthy() {
            // else cute if branch
            self.interpret(slice::from_ref(when_true))?;
        } else if let Some(stmt) = when_false {
            // execute else branch if defined
            self.interpret(slice::from_ref(stmt))?;
        }
        Ok(())
    }

    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> Self::Output {
        // capture condition variables separately
        self.env.begin_scope();

        while self
            .evaluate(condition)
            // clean up condition scope in case of failure
            .inspect_err(|_| self.env.end_scope())?
            .is_truthy()
        {
            // capture the body result
            self.env.begin_scope(); // capture while body scope
            let body_result = self.interpret(slice::from_ref(body));
            self.env.end_scope(); // drop while body scope

            match body_result {
                Err(RuntimeError::Continue) => continue,
                Err(RuntimeError::Break) => break,
                Err(e) => {
                    // early exit: drop while condition variables
                    self.env.end_scope();
                    return Err(e);
                }
                Ok(_) => {}
            };
        }

        // drop while loop condition variables
        self.env.end_scope();
        Ok(())
    }

    fn visit_continue(&mut self) -> Self::Output {
        Err(RuntimeError::Continue)
    }

    fn visit_break(&mut self) -> Self::Output {
        Err(RuntimeError::Break)
    }

    fn visit_for(
        &mut self,
        initializer: &Option<Box<Stmt>>,
        condition: &Option<Expr>,
        increment: &Option<Expr>,
        body: &Stmt,
    ) -> Self::Output {
        // capture for loop initializer in a new scope
        self.env.begin_scope();

        // run the initializer once
        if let Some(initializer) = initializer {
            self.interpret(slice::from_ref(initializer))
                .inspect_err(|_| {
                    // drop for loop condition variables
                    self.env.end_scope();
                })?;
        }

        while match condition {
            Some(expr) => self
                .evaluate(expr)
                // clean up condition scope in case of failure
                .inspect_err(|_| self.env.end_scope())?
                .is_truthy(),
            None => true,
        } {
            // capture the body result
            self.env.begin_scope(); // capture while body scope
            let body_result = self.interpret(slice::from_ref(body));
            self.env.end_scope(); // drop while body scope

            // run the increment expression for success & continue, but not break
            if let Some(increment) = increment {
                match body_result {
                    Err(RuntimeError::Continue) | Ok(_) => {
                        self.evaluate(increment)
                            // clean up body scope in case of failure
                            .inspect_err(|_| self.env.end_scope())?;
                    }
                    _ => {}
                }
            };

            // handle the result of the body evaluation
            match body_result {
                Err(RuntimeError::Continue) => continue,
                Err(RuntimeError::Break) => break,
                Err(e) => {
                    // early exit: drop while condition variables
                    self.env.end_scope();
                    return Err(e);
                }
                Ok(_) => {}
            };
        }
        // drop for loop condition variables
        self.env.end_scope();

        Ok(())
    }
}

impl ExprVisitor for Interpreter {
    type Output = InterpreterResult<Value>;

    fn visit_unary(&mut self, operator: &crate::frontend::Token, right: &Expr) -> Self::Output {
        let right_result = self.evaluate(right)?;

        match (&operator.token_type, right_result) {
            (TokenType::Minus, Value::Number(v)) => Ok(Value::Number(-v)),
            (TokenType::Bang, v) => Ok(Value::Bool(!v.is_truthy())),
            _ => Err(RuntimeError::InvalidOperation),
        }
    }

    fn visit_binary(
        &mut self,
        left: &Expr,
        operator: &crate::frontend::Token,
        right: &Expr,
    ) -> Self::Output {
        let left_result = self.evaluate(left)?;
        let right_resut = self.evaluate(right)?;

        match (&operator.token_type, left_result, right_resut) {
            // arithmetic
            (TokenType::Slash, Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
            (TokenType::Star, Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
            (TokenType::Minus, Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
            (TokenType::Plus, Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),

            // string concatenation
            (TokenType::Plus, Value::String(l), r) => Ok(Value::String(l + &r.to_string())),
            (TokenType::Plus, l, Value::String(r)) => Ok(Value::String(l.to_string() + &r)),

            // comparison
            (TokenType::Greater, Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l > r)),
            (TokenType::GreaterEqual, Value::Number(l), Value::Number(r)) => {
                Ok(Value::Bool(l >= r))
            }
            (TokenType::Less, Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l < r)),
            (TokenType::LessEqual, Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l <= r)),

            // equality - number
            (TokenType::EqualEqual, l, r) => Ok(Value::Bool(l == r)),
            (TokenType::BangEqual, l, r) => Ok(Value::Bool(l != r)),

            _ => Err(RuntimeError::InvalidOperation),
        }
    }

    fn visit_grouping(&mut self, expr: &Expr) -> Self::Output {
        self.evaluate(expr)
    }

    fn visit_variable(&mut self, name: &crate::frontend::Token) -> Self::Output {
        self.env
            .get(&name.lexeme)
            .map_err(|_| RuntimeError::UndefinedVariable {
                name: name.lexeme.clone(),
                at: name.span.offset.into(),
            })
            .cloned()
    }

    fn visit_assignment(&mut self, name: &crate::frontend::Token, value: &Expr) -> Self::Output {
        let result = self.evaluate(value)?;
        self.env
            .assign(&name.lexeme, &result)
            .map_err(|_| RuntimeError::UndefinedVariable {
                name: name.lexeme.clone(),
                at: name.span.offset.into(),
            })?;
        Ok(result)
    }

    fn visit_logical(
        &mut self,
        left: &Expr,
        operator: &crate::frontend::Token,
        right: &Expr,
    ) -> Self::Output {
        match operator.token_type {
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
        }
    }

    fn visit_bool(&mut self, value: &bool) -> Self::Output {
        Ok(Value::Bool(*value))
    }

    fn visit_number(&mut self, value: &f64) -> Self::Output {
        Ok(Value::Number(*value))
    }

    fn visit_string(&mut self, value: &str) -> Self::Output {
        Ok(Value::String(value.to_string()))
    }

    fn visit_nil(&mut self) -> Self::Output {
        Ok(Value::Nil)
    }
}
