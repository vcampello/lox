use crate::ast::{AstPrinter, Expr};
use crate::common::Span;
use crate::frontend::Token;

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

impl Stmt {
    pub fn print_stmt(expr: Expr) -> Self {
        Self {
            span: expr.span,
            kind: StmtKind::Print(expr),
        }
    }

    pub fn continue_stmt(span: Span) -> Self {
        Self {
            span,
            kind: StmtKind::Continue,
        }
    }

    pub fn break_stmt(span: Span) -> Self {
        Self {
            span,
            kind: StmtKind::Break,
        }
    }

    pub fn expr_stmt(expr: Expr) -> Self {
        Self {
            span: expr.span,
            kind: StmtKind::ExprStmt(expr),
        }
    }

    pub fn block_stmt(stmts: Vec<Stmt>, fallback_span: Span) -> Self {
        let at = match stmts
            .iter()
            .map(|stmt| stmt.span)
            .reduce(|acc, e| acc.merge(&e))
        {
            Some(combined_span) => combined_span,
            None => fallback_span,
        };

        Self {
            span: at,
            kind: StmtKind::Block(stmts),
        }
    }

    pub fn variable_stmt(var: Token, initializer: Option<Expr>) -> Self {
        let at = match &initializer {
            Some(expr) => var.span.merge(&expr.span),
            None => var.span,
        };

        Self {
            span: at,
            kind: StmtKind::Variable { var, initializer },
        }
    }

    pub fn conditional_stmt(condition: Expr, when_true: Stmt, when_false: Option<Stmt>) -> Self {
        let at = match &when_false {
            Some(else_branch) => condition
                .span
                .merge(&when_true.span)
                .merge(&else_branch.span),
            None => condition.span.merge(&when_true.span),
        };

        Self {
            span: at,
            kind: StmtKind::Conditional {
                condition,
                when_true: Box::new(when_true),
                when_false: when_false.map(Box::new),
            },
        }
    }

    pub fn while_loop(condition: Expr, body: Stmt) -> Self {
        Self {
            span: condition.span.merge(&body.span),
            kind: StmtKind::While {
                condition,
                body: Box::new(body),
            },
        }
    }

    pub fn for_loop(
        initializer: Option<Stmt>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Stmt,
    ) -> Self {
        let mut at = body.span;
        if let Some(ref stmt) = initializer {
            at = at.merge(&stmt.span)
        }

        if let Some(ref expr) = condition {
            at = at.merge(&expr.span)
        }

        if let Some(ref expr) = increment {
            at = at.merge(&expr.span)
        }

        Self {
            span: at,
            kind: StmtKind::For {
                initializer: initializer.map(Box::new),
                condition,
                increment,
                body: Box::new(body),
            },
        }
    }
}

// TODO: implement Deref as kind
#[derive(Debug, Clone)]
pub enum StmtKind {
    Block(Vec<Stmt>),
    ExprStmt(Expr),
    Print(Expr),
    Variable {
        // FIXME: why Token instead of something specific?
        var: Token,
        initializer: Option<Expr>,
    },
    Conditional {
        condition: Expr,
        when_true: Box<Stmt>,
        when_false: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Continue,
    Break,
    For {
        initializer: Option<Box<Stmt>>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Box<Stmt>,
    },
}

impl StmtKind {
    pub fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        walk_stmt(self, visitor)
    }
}

impl std::fmt::Display for StmtKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut printer = AstPrinter::new();
        write!(f, "{}", self.accept(&mut printer))
    }
}

pub trait StmtVisitor {
    type Output;

    fn visit_block(&mut self, stmts: &[Stmt]) -> Self::Output;

    fn visit_expression(&mut self, expr: &Expr) -> Self::Output;

    fn visit_print(&mut self, expr: &Expr) -> Self::Output;

    fn visit_variable(&mut self, var: &Token, initializer: &Option<Expr>) -> Self::Output;

    fn visit_conditional(
        &mut self,
        condition: &Expr,
        when_true: &Stmt,
        when_false: &Option<Box<Stmt>>,
    ) -> Self::Output;

    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> Self::Output;

    fn visit_continue(&mut self) -> Self::Output;

    fn visit_break(&mut self) -> Self::Output;

    fn visit_for(
        &mut self,
        initializer: &Option<Box<Stmt>>,
        condition: &Option<Expr>,
        increment: &Option<Expr>,
        body: &Stmt,
    ) -> Self::Output;
}

/// Default walking algorithm for statements
pub fn walk_stmt<V: StmtVisitor>(stmt: &StmtKind, visitor: &mut V) -> V::Output {
    match stmt {
        StmtKind::Block(stmts) => visitor.visit_block(stmts),
        StmtKind::ExprStmt(expr) => visitor.visit_expression(expr),
        StmtKind::Print(expr) => visitor.visit_print(expr),
        StmtKind::Variable { var, initializer } => visitor.visit_variable(var, initializer),
        StmtKind::Conditional {
            condition,
            when_true,
            when_false,
        } => visitor.visit_conditional(condition, when_true, when_false),
        StmtKind::While { condition, body } => visitor.visit_while(condition, body),
        StmtKind::Continue => visitor.visit_continue(),
        StmtKind::Break => visitor.visit_break(),
        StmtKind::For {
            initializer,
            condition,
            increment,
            body,
        } => visitor.visit_for(initializer, condition, increment, body),
    }
}
