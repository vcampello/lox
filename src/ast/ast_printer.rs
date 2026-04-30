use super::*;

#[derive(Debug, Default)]
pub struct AstPrinter {}

impl AstPrinter {
    pub fn new() -> Self {
        Self {}
    }
}

impl ExprVisitor for AstPrinter {
    type Output = String;

    fn visit_number(&mut self, expr: &NumberLiteralExpr) -> String {
        expr.value.to_string()
    }

    fn visit_string(&mut self, expr: &StringLiteralExpr) -> String {
        format!("\"{}\"", expr.value)
    }

    fn visit_bool(&mut self, expr: &BoolLiteralExpr) -> String {
        expr.value.to_string()
    }

    fn visit_nil(&mut self, _expr: &NilExpr) -> String {
        "nil".to_string()
    }

    fn visit_grouping(&mut self, expr: &GroupingExpr) -> String {
        format!("(group {})", expr.group.visit(self))
    }

    fn visit_unary(&mut self, expr: &UnaryExpr) -> String {
        format!("({} {})", expr.operator.value, expr.right.visit(self))
    }

    fn visit_binary(&mut self, expr: &BinaryExpr) -> String {
        format!(
            "({} {} {})",
            expr.operator.value,
            expr.left.visit(self),
            expr.right.visit(self)
        )
    }

    fn visit_variable(&mut self, expr: &VariableExpr) -> String {
        expr.var.lexeme.clone()
    }

    fn visit_assignment(&mut self, expr: &AssignmentExpr) -> String {
        format!("(= {} {})", expr.name.lexeme, expr.value.visit(self))
    }

    fn visit_logical(&mut self, expr: &LogicalExpr) -> String {
        format!(
            "({} {} {})",
            expr.operator.value,
            expr.left.visit(self),
            expr.right.visit(self)
        )
    }

    fn visit_call(&mut self, _expr: &CallExpr) -> Self::Output {
        // enhance this once the printer is revisited
        "call".to_string()
    }
}

impl StmtVisitor for AstPrinter {
    type Output = String;

    fn visit_block(&mut self, stmt: &BlockStmt) -> Self::Output {
        let body = stmt
            .stmts
            .iter()
            .map(|stmt| stmt.visit(self))
            .collect::<Vec<_>>()
            .join(" ");
        format!("(block {})", body)
    }

    fn visit_expr_stmt(&mut self, stmt: &ExprStmt) -> Self::Output {
        stmt.expr.visit(self)
    }

    fn visit_print(&mut self, stmt: &PrintStmt) -> Self::Output {
        format!("(print {})", stmt.expr.visit(self))
    }

    fn visit_variable(&mut self, stmt: &VariableStmt) -> Self::Output {
        match &stmt.initializer {
            Some(expr) => format!("(var {} = {})", stmt.name, expr.visit(self)),
            None => format!("(var {})", stmt.name),
        }
    }

    fn visit_conditional(&mut self, stmt: &ConditionalStmt) -> Self::Output {
        let cond = stmt.condition.visit(self);
        let true_result = stmt.when_true.visit(self);

        match stmt.when_false.as_ref().map(|stmt| stmt.visit(self)) {
            Some(false_result) => format!("(if {} {} {})", cond, true_result, false_result),
            None => format!("(if {} {})", cond, true_result),
        }
    }

    fn visit_while(&mut self, stmt: &WhileStmt) -> Self::Output {
        let cond = stmt.condition.visit(self);

        format!("(while {} {})", cond, stmt.body.visit(self))
    }

    fn visit_continue(&mut self, _stmt: &ContinueStmt) -> Self::Output {
        "continue".to_string()
    }

    fn visit_break(&mut self, _stmt: &BreakStmt) -> Self::Output {
        "break".to_string()
    }

    fn visit_for(&mut self, stmt: &ForStmt) -> Self::Output {
        let init = stmt
            .initializer
            .as_ref()
            .map(|stmt| stmt.visit(self))
            .unwrap_or("_".to_string());

        let cond = stmt
            .condition
            .as_ref()
            .map(|expr| expr.visit(self))
            .unwrap_or("_".to_string());

        let inc = stmt
            .increment
            .as_ref()
            .map(|expr| expr.visit(self))
            .unwrap_or("_".to_string());

        format!(
            "(for {}; {}; {}; {})",
            init,
            cond,
            inc,
            stmt.body.visit(self)
        )
    }

    fn visit_function(&mut self, stmt: &FunctionStmt) -> Self::Output {
        format!("<function {}>", stmt.name)
    }

    fn visit_return(&mut self, stmt: &ReturnStmt) -> Self::Output {
        match &stmt.value {
            Some(stmt) => format!("<return {}>", stmt.visit(self)),
            None => format!("<return nil>"),
        }
    }
}
