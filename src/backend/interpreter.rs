use super::*;
use crate::{
    ast::{BinaryOp, Expr, ExprVisitor, LogicalOp, Stmt, StmtVisitor, Token, UnaryOp},
    common::{Span, Spanned},
};

pub type InterpreterResult<T> = Result<T, RuntimeError>;

#[derive(Debug, Default)]
pub struct Interpreter {
    env: Env,
    // REVIEW: there must be a better way to track the Span for continue and break
    /// Represents the last known statement spam (used to signal break and continue)
    last_stmt_span: Span,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Env::new(),
            last_stmt_span: Span::default(),
        }
    }

    pub fn interpret(&mut self, stmts: &[Stmt]) -> InterpreterResult<()> {
        for stmt in stmts.iter() {
            self.last_stmt_span = stmt.span;
            self.visit_stmt(stmt)?;
        }

        Ok(())
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
        self.visit_expr(expr)?;
        Ok(())
    }

    fn visit_print(&mut self, expr: &Expr) -> Self::Output {
        let result = self.visit_expr(expr)?;
        println!("{result}");
        Ok(())
    }

    fn visit_variable(&mut self, var: &Token, initializer: &Option<Expr>) -> Self::Output {
        let value = match initializer {
            Some(expr) => self.visit_expr(expr)?,
            None => Value::Nil,
        };
        self.env.define(&var.lexeme, &value);
        Ok(())
    }

    fn visit_conditional(
        &mut self,
        condition: &Expr,
        when_true: &Stmt,
        when_false: &Option<Box<Stmt>>,
    ) -> Self::Output {
        if self.visit_expr(condition)?.is_truthy() {
            // else cute if branch
            self.visit_stmt(when_true)?;
        } else if let Some(stmt) = when_false {
            // execute else branch if defined
            self.visit_stmt(stmt)?;
        }

        Ok(())
    }

    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> Self::Output {
        // capture condition variables separately
        self.env.begin_scope();

        while self
            .visit_expr(condition)
            // clean up condition scope in case of failure
            .inspect_err(|_| self.env.end_scope())?
            .is_truthy()
        {
            // capture the body result
            self.env.begin_scope(); // capture while body scope
            let body_result = self.visit_stmt(body);
            self.env.end_scope(); // drop while body scope

            if let Err(e) = body_result {
                match e.kind {
                    RuntimeErrorKind::Continue => continue,
                    RuntimeErrorKind::Break => break,
                    _ => {
                        // early exit: drop while condition variables
                        self.env.end_scope();
                        return Err(e);
                    }
                };
            }
        }

        // drop while loop condition variables
        self.env.end_scope();
        Ok(())
    }

    fn visit_continue(&mut self) -> Self::Output {
        Err(RuntimeError::continue_signal(self.last_stmt_span))
    }

    fn visit_break(&mut self) -> Self::Output {
        Err(RuntimeError::break_signal(self.last_stmt_span))
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
            self.visit_stmt(initializer).inspect_err(|_| {
                // drop for loop condition variables
                self.env.end_scope();
            })?;
        }

        while match condition {
            Some(expr) => self
                .visit_expr(expr)
                // clean up condition scope in case of failure
                .inspect_err(|_| self.env.end_scope())?
                .is_truthy(),
            None => true,
        } {
            // capture the body result
            self.env.begin_scope(); // capture while body scope
            let body_result = self.visit_stmt(body);
            self.env.end_scope(); // drop while body scope

            // run the increment expression for success & continue, but not break
            if let Some(increment) = increment {
                // REVIEW: is there a nicer way to write this?
                match body_result {
                    Err(RuntimeError {
                        kind: RuntimeErrorKind::Continue,
                        ..
                    })
                    | Ok(_) => {
                        self.visit_expr(increment)
                            // clean up body scope in case of failure
                            .inspect_err(|_| self.env.end_scope())?;
                    }
                    _ => {}
                }
            };

            // handle the result of the body evaluation
            if let Err(e) = body_result {
                match e.kind {
                    RuntimeErrorKind::Continue => continue,
                    RuntimeErrorKind::Break => break,
                    _ => {
                        // early exit: drop while condition variables
                        self.env.end_scope();
                        return Err(e);
                    }
                };
            }
        }
        // drop for loop condition variables
        self.env.end_scope();

        Ok(())
    }
}

impl ExprVisitor for Interpreter {
    type Output = InterpreterResult<Value>;

    fn visit_unary(&mut self, operator: &Spanned<UnaryOp>, right: &Expr) -> Self::Output {
        let right_result = self.visit_expr(right)?;

        match (&operator.value, right_result) {
            (UnaryOp::Neg, Value::Number(v)) => Ok(Value::Number(-v)),
            (UnaryOp::Not, v) => Ok(Value::Bool(!v.is_truthy())),
            _ => Err(RuntimeError::invalid_op(
                operator.to_string(),
                operator.span.merge(&right.span),
            )),
        }
    }

    fn visit_binary(
        &mut self,
        left: &Expr,
        operator: &Spanned<BinaryOp>,
        right: &Expr,
    ) -> Self::Output {
        let l_val = self.visit_expr(left)?;
        let r_val = self.visit_expr(right)?;

        match (&operator.value, l_val, r_val) {
            // arithmetic
            (BinaryOp::Div, Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
            (BinaryOp::Mul, Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
            (BinaryOp::Sub, Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
            (BinaryOp::Add, Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),

            // string concatenation
            (BinaryOp::Add, Value::String(l), r) => Ok(Value::String(l + &r.to_string())),
            (BinaryOp::Add, l, Value::String(r)) => Ok(Value::String(l.to_string() + &r)),

            // comparison
            (BinaryOp::Greater, Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l > r)),
            (BinaryOp::GreaterEqual, Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l >= r)),
            (BinaryOp::Less, Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l < r)),
            (BinaryOp::LessEqual, Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l <= r)),

            // equality - number
            (BinaryOp::EqualEqual, l, r) => Ok(Value::Bool(l == r)),
            (BinaryOp::BangEqual, l, r) => Ok(Value::Bool(l != r)),

            (op, ..) => Err(RuntimeError::invalid_op(
                op.to_string(),
                left.span.merge(&operator.span).merge(&right.span),
            )),
        }
    }

    fn visit_grouping(&mut self, expr: &Expr) -> Self::Output {
        self.visit_expr(expr)
    }

    fn visit_variable(&mut self, name: &Token) -> Self::Output {
        // let span = &name.span;
        self.env
            .get(&name.lexeme)
            .map_err(|e| match e {
                EnvironmentError::UndefinedVariable { name: var_name } => {
                    RuntimeError::undefined_var(var_name, name.span)
                }
            })
            .cloned()
    }

    fn visit_assignment(&mut self, name: &Token, value: &Expr) -> Self::Output {
        let result = self.visit_expr(value)?;
        self.env
            .assign(&name.lexeme, &result)
            .map_err(|e| match e {
                EnvironmentError::UndefinedVariable { name: var_name } => {
                    RuntimeError::undefined_var(var_name, name.span)
                }
            })?;
        Ok(result)
    }

    fn visit_logical(
        &mut self,
        left: &Expr,
        operator: &Spanned<LogicalOp>,
        right: &Expr,
    ) -> Self::Output {
        match operator.value {
            LogicalOp::And => {
                let left_result = self.visit_expr(left)?;
                match left_result.is_truthy() {
                    // short circuit
                    false => Ok(left_result),
                    // keep chaining so long as it's true
                    true => self.visit_expr(right),
                }
            }
            LogicalOp::Or => {
                let left_result = self.visit_expr(left)?;
                match left_result.is_truthy() {
                    // short circuit
                    true => Ok(left_result),
                    // keep chaining
                    false => self.visit_expr(right),
                }
            }
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
