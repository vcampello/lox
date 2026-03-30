use crate::ast::{AstPrinter, ExprKind};
use crate::frontend::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(ExprKind),
    Print(ExprKind),
    Variable {
        name: Token,
        initializer: Option<ExprKind>,
    },
    Conditional {
        condition: ExprKind,
        when_true: Box<Stmt>,
        when_false: Option<Box<Stmt>>,
    },
    While {
        condition: ExprKind,
        body: Box<Stmt>,
    },
    Continue,
    Break,
    For {
        initializer: Option<Box<Stmt>>,
        condition: Option<ExprKind>,
        increment: Option<ExprKind>,
        body: Box<Stmt>,
    },
}

impl Stmt {
    pub fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        walk_stmt(self, visitor)
    }

    pub fn new_conditional(condition: ExprKind, when_true: Stmt, when_false: Option<Stmt>) -> Self {
        Self::Conditional {
            condition,
            when_true: Box::new(when_true),
            when_false: when_false.map(Box::new),
        }
    }

    pub fn new_while(condition: ExprKind, body: Stmt) -> Self {
        Self::While {
            condition,
            body: Box::new(body),
        }
    }

    pub fn new_for(
        initializer: Option<Stmt>,
        condition: Option<ExprKind>,
        increment: Option<ExprKind>,
        body: Stmt,
    ) -> Self {
        Self::For {
            initializer: initializer.map(Box::new),
            condition,
            increment,
            body: Box::new(body),
        }
    }
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut printer = AstPrinter::new();
        write!(f, "{}", self.accept(&mut printer))
    }
}

pub trait StmtVisitor {
    type Output;

    fn visit_block(&mut self, stmts: &[Stmt]) -> Self::Output;

    fn visit_expression(&mut self, expr: &ExprKind) -> Self::Output;

    fn visit_print(&mut self, expr: &ExprKind) -> Self::Output;

    fn visit_variable(&mut self, name: &Token, initializer: &Option<ExprKind>) -> Self::Output;

    fn visit_conditional(
        &mut self,
        condition: &ExprKind,
        when_true: &Stmt,
        when_false: &Option<Box<Stmt>>,
    ) -> Self::Output;

    fn visit_while(&mut self, condition: &ExprKind, body: &Stmt) -> Self::Output;

    fn visit_continue(&mut self) -> Self::Output;

    fn visit_break(&mut self) -> Self::Output;

    fn visit_for(
        &mut self,
        initializer: &Option<Box<Stmt>>,
        condition: &Option<ExprKind>,
        increment: &Option<ExprKind>,
        body: &Stmt,
    ) -> Self::Output;
}

/// Default walking algorithm for statements
pub fn walk_stmt<V: StmtVisitor>(stmt: &Stmt, visitor: &mut V) -> V::Output {
    match stmt {
        Stmt::Block(stmts) => visitor.visit_block(stmts),
        Stmt::Expression(expr) => visitor.visit_expression(expr),
        Stmt::Print(expr) => visitor.visit_print(expr),
        Stmt::Variable { name, initializer } => visitor.visit_variable(name, initializer),
        Stmt::Conditional {
            condition,
            when_true,
            when_false,
        } => visitor.visit_conditional(condition, when_true, when_false),
        Stmt::While { condition, body } => visitor.visit_while(condition, body),
        Stmt::Continue => visitor.visit_continue(),
        Stmt::Break => visitor.visit_break(),
        Stmt::For {
            initializer,
            condition,
            increment,
            body,
        } => visitor.visit_for(initializer, condition, increment, body),
    }
}
