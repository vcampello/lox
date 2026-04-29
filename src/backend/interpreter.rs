use super::*;
use crate::ast::*;

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
            self.visit_stmt(stmt)?;
        }

        Ok(())
    }
}

impl StmtVisitor for Interpreter {
    type Output = InterpreterResult<()>;

    fn visit_block(&mut self, stmt: &BlockStmt) -> Self::Output {
        // new scope for the upcoming block
        self.env.begin_scope();

        // the scope needs to be dropped regardless of the result
        let result = self.interpret(&stmt.stmts);
        self.env.end_scope();

        result
    }

    fn visit_expr_stmt(&mut self, stmt: &ExprStmt) -> Self::Output {
        self.visit_expr(&stmt.expr)?;
        Ok(())
    }

    fn visit_print(&mut self, stmt: &PrintStmt) -> Self::Output {
        let result = self.visit_expr(&stmt.expr)?;
        println!("{result}");
        Ok(())
    }

    fn visit_variable(&mut self, stmt: &VariableStmt) -> Self::Output {
        let value = match &stmt.initializer {
            Some(expr) => self.visit_expr(expr)?,
            None => Value::Nil,
        };
        self.env.define(&stmt.name, &value);
        Ok(())
    }

    fn visit_conditional(&mut self, stmt: &ConditionalStmt) -> Self::Output {
        if self.visit_expr(&stmt.condition)?.is_truthy() {
            // else cute if branch
            self.visit_stmt(&stmt.when_true)?;
        } else if let Some(stmt) = &stmt.when_false {
            // execute else branch if defined
            self.visit_stmt(stmt)?;
        }

        Ok(())
    }

    fn visit_while(&mut self, stmt: &WhileStmt) -> Self::Output {
        // capture condition variables separately
        self.env.begin_scope();

        while self
            .visit_expr(&stmt.condition)
            // clean up condition scope in case of failure
            .inspect_err(|_| self.env.end_scope())?
            .is_truthy()
        {
            // capture the body result
            self.env.begin_scope(); // capture while body scope
            let body_result = self.visit_stmt(&stmt.body);
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

    fn visit_continue(&mut self, stmt: &ContinueStmt) -> Self::Output {
        Err(RuntimeError::continue_signal(stmt.span))
    }

    fn visit_break(&mut self, stmt: &BreakStmt) -> Self::Output {
        Err(RuntimeError::break_signal(stmt.span))
    }

    fn visit_for(&mut self, stmt: &ForStmt) -> Self::Output {
        // capture for loop initializer in a new scope
        self.env.begin_scope();

        // run the initializer once
        if let Some(initializer) = &stmt.initializer {
            self.visit_stmt(initializer).inspect_err(|_| {
                // drop for loop condition variables
                self.env.end_scope();
            })?;
        }

        while match &stmt.condition {
            Some(expr) => self
                .visit_expr(expr)
                // clean up condition scope in case of failure
                .inspect_err(|_| self.env.end_scope())?
                .is_truthy(),
            None => true,
        } {
            // capture the body result
            self.env.begin_scope(); // capture while body scope
            let body_result = self.visit_stmt(&stmt.body);
            self.env.end_scope(); // drop while body scope

            // run the increment expression for success & continue, but not break
            if let Some(increment) = &stmt.increment {
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

    fn visit_function(&mut self, stmt: &FunctionStmt) -> Self::Output {
        let declaration = Value::Function(stmt.clone());
        self.env.define(&stmt.name.lexeme, &declaration);
        Ok(())
    }
}

impl ExprVisitor for Interpreter {
    type Output = InterpreterResult<Value>;

    fn visit_unary(&mut self, expr: &UnaryExpr) -> Self::Output {
        let right_result = self.visit_expr(&expr.right)?;

        match (&expr.operator.value, right_result) {
            (UnaryOp::Neg, Value::Number(v)) => Ok(Value::Number(-v)),
            (UnaryOp::Not, v) => Ok(Value::Bool(!v.is_truthy())),
            _ => Err(RuntimeError::invalid_op(
                expr.operator.to_string(),
                expr.span,
            )),
        }
    }

    fn visit_binary(&mut self, expr: &BinaryExpr) -> Self::Output {
        let l_val = self.visit_expr(&expr.left)?;
        let r_val = self.visit_expr(&expr.right)?;

        match (&expr.operator.value, l_val, r_val) {
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

            (op, ..) => Err(RuntimeError::invalid_op(op.to_string(), expr.span)),
        }
    }

    fn visit_grouping(&mut self, expr: &GroupingExpr) -> Self::Output {
        self.visit_expr(&expr.group)
    }

    fn visit_variable(&mut self, expr: &VariableExpr) -> Self::Output {
        self.env
            .get(&expr.var.lexeme)
            .map_err(|e| match e {
                EnvironmentError::UndefinedVariable { name: var_name } => {
                    RuntimeError::undefined_var(var_name, expr.span)
                }
            })
            .cloned()
    }

    fn visit_assignment(&mut self, expr: &AssignmentExpr) -> Self::Output {
        let result = self.visit_expr(&expr.value)?;
        self.env
            .assign(&expr.name.lexeme, &result)
            .map_err(|e| match e {
                EnvironmentError::UndefinedVariable { name: var_name } => {
                    RuntimeError::undefined_var(var_name, expr.span)
                }
            })?;
        Ok(result)
    }

    fn visit_logical(&mut self, expr: &LogicalExpr) -> Self::Output {
        match expr.operator.value {
            LogicalOp::And => {
                let left_result = self.visit_expr(&expr.left)?;
                match left_result.is_truthy() {
                    // short circuit
                    false => Ok(left_result),
                    // keep chaining so long as it's true
                    true => self.visit_expr(&expr.right),
                }
            }
            LogicalOp::Or => {
                let left_result = self.visit_expr(&expr.left)?;
                match left_result.is_truthy() {
                    // short circuit
                    true => Ok(left_result),
                    // keep chaining
                    false => self.visit_expr(&expr.right),
                }
            }
        }
    }

    fn visit_bool(&mut self, expr: &BoolLiteralExpr) -> Self::Output {
        Ok(Value::Bool(expr.value))
    }

    fn visit_number(&mut self, expr: &NumberLiteralExpr) -> Self::Output {
        Ok(Value::Number(expr.value))
    }

    fn visit_string(&mut self, expr: &StringLiteralExpr) -> Self::Output {
        Ok(Value::String(expr.value.to_string()))
    }

    fn visit_nil(&mut self, _expr: &NilExpr) -> Self::Output {
        Ok(Value::Nil)
    }

    fn visit_call(&mut self, expr: &CallExpr) -> Self::Output {
        // fetch function definition from environment
        let callee = self.visit_expr(&expr.callee)?;

        // extract function
        let func = match callee {
            Value::Function(v) => v,
            // should this be the span of callee instead?
            _ => return Err(RuntimeError::not_callable(expr.callee.span)),
        };

        // validate that the number of arguments is correct
        if expr.arguments.len() != func.params.len() {
            return Err(RuntimeError::incorrect_arity(
                func.params.len(),
                expr.arguments.len(),
                expr.span,
            ));
        }

        // Bind parameters
        for (param, argument) in func.params.iter().zip(expr.arguments.iter()) {
            // evaluate argument
            let value = self.visit_expr(argument)?;
            // assign to function scope
            self.env.define(&param.lexeme, &value);
        }

        // execute body
        self.env.begin_scope();
        // TODO: drop the result for now. This will need to be revisited for the return statement
        _ = self.visit_stmt(&func.body);
        self.env.end_scope();

        Ok(Value::Nil)
    }
}
