use std::ops::Deref;

use super::Token;
use crate::ast::{AstPrinter, Expr};
use crate::common::Span;

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

impl Stmt {
    pub fn print_stmt(expr: Expr) -> Self {
        PrintStmt::new(expr).into()
    }

    pub fn continue_stmt(span: Span) -> Self {
        ContinueStmt::new(span).into()
    }

    pub fn break_stmt(span: Span) -> Self {
        BreakStmt::new(span).into()
    }

    pub fn expr_stmt(expr: Expr) -> Self {
        ExprStmt::new(expr).into()
    }

    pub fn block_stmt(stmts: Vec<Stmt>, fallback_span: Span) -> Self {
        BlockStmt::new(stmts, fallback_span).into()
    }

    pub fn variable_stmt(var: Token, initializer: Option<Expr>) -> Self {
        VariableStmt::new(var, initializer).into()
    }

    pub fn conditional_stmt(condition: Expr, when_true: Stmt, when_false: Option<Stmt>) -> Self {
        ConditionalStmt::new(condition, when_true, when_false).into()
    }

    pub fn while_loop(condition: Expr, body: Stmt) -> Self {
        WhileStmt::new(condition, body).into()
    }

    pub fn for_loop(
        initializer: Option<Stmt>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Stmt,
    ) -> Self {
        ForStmt::new(initializer, condition, increment, body).into()
    }
}

impl Deref for Stmt {
    type Target = StmtKind;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    Block(BlockStmt),
    ExprStmt(ExprStmt),
    Print(PrintStmt),
    Variable(VariableStmt),
    Conditional(ConditionalStmt),
    While(WhileStmt),
    Continue(ContinueStmt),
    Break(BreakStmt),
    For(ForStmt),
}

impl StmtKind {
    pub fn visit<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        walk_stmt(self, visitor)
    }
}

impl std::fmt::Display for StmtKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut printer = AstPrinter::new();
        write!(f, "{}", self.visit(&mut printer))
    }
}

pub trait StmtVisitor {
    type Output;

    /// Defines the default statement evaluation algorithm
    fn visit_stmt(&mut self, stmt: &Stmt) -> Self::Output
    where
        Self: Sized,
    {
        walk_stmt(&stmt.kind, self)
    }

    fn visit_block(&mut self, stmt: &BlockStmt) -> Self::Output;

    fn visit_expr_stmt(&mut self, stmt: &ExprStmt) -> Self::Output;

    fn visit_print(&mut self, stmt: &PrintStmt) -> Self::Output;

    fn visit_variable(&mut self, stmt: &VariableStmt) -> Self::Output;

    fn visit_conditional(&mut self, stmt: &ConditionalStmt) -> Self::Output;

    fn visit_while(&mut self, stmt: &WhileStmt) -> Self::Output;

    fn visit_continue(&mut self, stmt: &ContinueStmt) -> Self::Output;

    fn visit_break(&mut self, stmt: &BreakStmt) -> Self::Output;

    fn visit_for(&mut self, stmt: &ForStmt) -> Self::Output;
}

/// Default walking algorithm for statements
pub fn walk_stmt<V: StmtVisitor>(stmt: &StmtKind, visitor: &mut V) -> V::Output {
    match stmt {
        StmtKind::Block(s) => visitor.visit_block(s),
        StmtKind::ExprStmt(s) => visitor.visit_expr_stmt(s),
        StmtKind::Print(s) => visitor.visit_print(s),
        StmtKind::Variable(s) => visitor.visit_variable(s),
        StmtKind::Conditional(s) => visitor.visit_conditional(s),
        StmtKind::While(s) => visitor.visit_while(s),
        StmtKind::Continue(s) => visitor.visit_continue(s),
        StmtKind::Break(s) => visitor.visit_break(s),
        StmtKind::For(s) => visitor.visit_for(s),
    }
}

#[derive(Debug, Clone)]
pub struct BlockStmt {
    pub span: Span,
    pub stmts: Vec<Stmt>,
}

impl BlockStmt {
    pub fn new(stmts: Vec<Stmt>, fallback_span: Span) -> Self {
        let span = match stmts
            .iter()
            .map(|stmt| stmt.span)
            .reduce(|acc, e| acc.merge(&e))
        {
            Some(combined_span) => combined_span,
            None => fallback_span,
        };

        Self { span, stmts }
    }
}

impl From<BlockStmt> for Stmt {
    fn from(value: BlockStmt) -> Self {
        Self {
            span: value.span,
            kind: StmtKind::Block(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExprStmt {
    pub span: Span,
    pub expr: Expr,
}

impl ExprStmt {
    pub fn new(expr: Expr) -> Self {
        Self {
            span: expr.span,
            expr,
        }
    }
}

impl From<ExprStmt> for Stmt {
    fn from(value: ExprStmt) -> Self {
        Self {
            span: value.span,
            kind: StmtKind::ExprStmt(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrintStmt {
    pub span: Span,
    pub expr: Expr,
}

impl PrintStmt {
    pub fn new(expr: Expr) -> Self {
        Self {
            span: expr.span,
            expr,
        }
    }
}

impl From<PrintStmt> for Stmt {
    fn from(value: PrintStmt) -> Self {
        Self {
            span: value.span,
            kind: StmtKind::Print(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VariableStmt {
    pub span: Span,
    pub name: String,
    pub initializer: Option<Expr>,
}

impl VariableStmt {
    pub fn new(var: Token, initializer: Option<Expr>) -> Self {
        let span = match &initializer {
            Some(expr) => var.span.merge(&expr.span),
            None => var.span,
        };

        Self {
            span,
            name: var.lexeme,
            initializer,
        }
    }
}

impl From<VariableStmt> for Stmt {
    fn from(value: VariableStmt) -> Self {
        Self {
            span: value.span,
            kind: StmtKind::Variable(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConditionalStmt {
    pub span: Span,
    pub condition: Expr,
    pub when_true: Box<Stmt>,
    pub when_false: Option<Box<Stmt>>,
}

impl ConditionalStmt {
    pub fn new(condition: Expr, when_true: Stmt, when_false: Option<Stmt>) -> Self {
        let span = match &when_false {
            Some(else_branch) => condition
                .span
                .merge(&when_true.span)
                .merge(&else_branch.span),
            None => condition.span.merge(&when_true.span),
        };

        Self {
            span,
            condition,
            when_true: Box::new(when_true),
            when_false: when_false.map(Box::new),
        }
    }
}

impl From<ConditionalStmt> for Stmt {
    fn from(value: ConditionalStmt) -> Self {
        Self {
            span: value.span,
            kind: StmtKind::Conditional(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub span: Span,
    pub condition: Expr,
    pub body: Box<Stmt>,
}

impl WhileStmt {
    pub fn new(condition: Expr, body: Stmt) -> Self {
        let span = condition.span.merge(&body.span);
        Self {
            span,
            condition,
            body: Box::new(body),
        }
    }
}

impl From<WhileStmt> for Stmt {
    fn from(value: WhileStmt) -> Self {
        Self {
            span: value.span,
            kind: StmtKind::While(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContinueStmt {
    pub span: Span,
}

impl ContinueStmt {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

impl From<ContinueStmt> for Stmt {
    fn from(value: ContinueStmt) -> Self {
        Self {
            span: value.span,
            kind: StmtKind::Continue(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BreakStmt {
    pub span: Span,
}

impl BreakStmt {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

impl From<BreakStmt> for Stmt {
    fn from(value: BreakStmt) -> Self {
        Self {
            span: value.span,
            kind: StmtKind::Break(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ForStmt {
    pub span: Span,
    pub initializer: Option<Box<Stmt>>,
    pub condition: Option<Expr>,
    pub increment: Option<Expr>,
    pub body: Box<Stmt>,
}

impl ForStmt {
    pub fn new(
        initializer: Option<Stmt>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Stmt,
    ) -> Self {
        let mut span = body.span;
        if let Some(ref stmt) = initializer {
            span = span.merge(&stmt.span)
        }

        if let Some(ref expr) = condition {
            span = span.merge(&expr.span)
        }

        if let Some(ref expr) = increment {
            span = span.merge(&expr.span)
        }

        Self {
            span,
            initializer: initializer.map(Box::new),
            condition,
            increment,
            body: Box::new(body),
        }
    }
}

impl From<ForStmt> for Stmt {
    fn from(value: ForStmt) -> Self {
        Self {
            span: value.span,
            kind: StmtKind::For(value),
        }
    }
}
