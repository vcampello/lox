use super::{Expr, ExprVisitor, Stmt, StmtVisitor};
use crate::frontend::Token;

#[derive(Debug, Default)]
pub struct AstPrinter {}

impl AstPrinter {
    pub fn new() -> Self {
        Self {}
    }
}

impl ExprVisitor for AstPrinter {
    type Output = String;

    fn visit_number(&mut self, value: &f64) -> String {
        value.to_string()
    }

    fn visit_string(&mut self, value: &str) -> String {
        format!("\"{}\"", value)
    }

    fn visit_bool(&mut self, value: &bool) -> String {
        value.to_string()
    }

    fn visit_nil(&mut self) -> String {
        "nil".to_string()
    }

    fn visit_grouping(&mut self, expr: &Expr) -> String {
        format!("(group {})", expr.kind.accept(self))
    }

    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> String {
        format!("({} {})", operator.lexeme, right.kind.accept(self))
    }

    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> String {
        format!(
            "({} {} {})",
            operator.lexeme,
            left.kind.accept(self),
            right.kind.accept(self)
        )
    }

    fn visit_variable(&mut self, name: &Token) -> String {
        name.lexeme.clone()
    }

    fn visit_assignment(&mut self, name: &Token, value: &Expr) -> String {
        format!("(= {} {})", name.lexeme, value.kind.accept(self))
    }

    fn visit_logical(&mut self, left: &Expr, operator: &Token, right: &Expr) -> String {
        format!(
            "({} {} {})",
            operator.lexeme,
            left.kind.accept(self),
            right.kind.accept(self)
        )
    }
}

impl StmtVisitor for AstPrinter {
    type Output = String;

    fn visit_block(&mut self, stmts: &[Stmt]) -> Self::Output {
        let body = stmts
            .iter()
            .map(|stmt| stmt.accept(self))
            .collect::<Vec<_>>()
            .join(" ");
        format!("(block {})", body)
    }

    fn visit_expression(&mut self, expr: &Expr) -> Self::Output {
        expr.kind.accept(self)
    }

    fn visit_print(&mut self, expr: &Expr) -> Self::Output {
        format!("(print {})", expr.kind.accept(self))
    }

    fn visit_variable(&mut self, var: &Token, initializer: &Option<Expr>) -> Self::Output {
        match initializer {
            Some(expr) => format!("(var {} = {})", var.lexeme, expr.kind.accept(self)),
            None => format!("(var {})", var.lexeme),
        }
    }

    fn visit_conditional(
        &mut self,
        condition: &Expr,
        when_true: &Stmt,
        when_false: &Option<Box<Stmt>>,
    ) -> Self::Output {
        let cond = condition.kind.accept(self);
        let true_result = when_true.accept(self);

        match when_false.as_ref().map(|stmt| stmt.accept(self)) {
            Some(false_result) => format!("(if {} {} {})", cond, true_result, false_result),
            None => format!("(if {} {})", cond, true_result),
        }
    }

    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> Self::Output {
        let cond = condition.kind.accept(self);

        format!("(while {} {})", cond, body.accept(self))
    }

    fn visit_continue(&mut self) -> Self::Output {
        "continue".to_string()
    }

    fn visit_break(&mut self) -> Self::Output {
        "break".to_string()
    }

    fn visit_for(
        &mut self,
        initializer: &Option<Box<Stmt>>,
        condition: &Option<Expr>,
        increment: &Option<Expr>,
        body: &Stmt,
    ) -> Self::Output {
        let init = initializer
            .as_ref()
            .map(|stmt| stmt.accept(self))
            .unwrap_or("_".to_string());

        let cond = condition
            .as_ref()
            .map(|expr| expr.kind.accept(self))
            .unwrap_or("_".to_string());

        let inc = increment
            .as_ref()
            .map(|expr| expr.kind.accept(self))
            .unwrap_or("_".to_string());

        format!("(for {}; {}; {}; {})", init, cond, inc, body.accept(self))
    }
}
