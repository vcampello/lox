use crate::ast::{AstPrinter, Expr};
use crate::frontend::Token;

// TODO: implement Deref as kind
#[derive(Debug, Clone)]
pub enum StmtKind {
    Block(Vec<StmtKind>),
    Expression(Expr),
    Print(Expr),
    Variable {
        name: Token,
        initializer: Option<Expr>,
    },
    Conditional {
        condition: Expr,
        when_true: Box<StmtKind>,
        when_false: Option<Box<StmtKind>>,
    },
    While {
        condition: Expr,
        body: Box<StmtKind>,
    },
    Continue,
    Break,
    For {
        initializer: Option<Box<StmtKind>>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Box<StmtKind>,
    },
}

impl StmtKind {
    pub fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        walk_stmt(self, visitor)
    }

    pub fn new_conditional(
        condition: Expr,
        when_true: StmtKind,
        when_false: Option<StmtKind>,
    ) -> Self {
        Self::Conditional {
            condition,
            when_true: Box::new(when_true),
            when_false: when_false.map(Box::new),
        }
    }

    pub fn new_while(condition: Expr, body: StmtKind) -> Self {
        Self::While {
            condition,
            body: Box::new(body),
        }
    }

    pub fn new_for(
        initializer: Option<StmtKind>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: StmtKind,
    ) -> Self {
        Self::For {
            initializer: initializer.map(Box::new),
            condition,
            increment,
            body: Box::new(body),
        }
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

    fn visit_block(&mut self, stmts: &[StmtKind]) -> Self::Output;

    fn visit_expression(&mut self, expr: &Expr) -> Self::Output;

    fn visit_print(&mut self, expr: &Expr) -> Self::Output;

    fn visit_variable(&mut self, var: &Token, initializer: &Option<Expr>) -> Self::Output;

    fn visit_conditional(
        &mut self,
        condition: &Expr,
        when_true: &StmtKind,
        when_false: &Option<Box<StmtKind>>,
    ) -> Self::Output;

    fn visit_while(&mut self, condition: &Expr, body: &StmtKind) -> Self::Output;

    fn visit_continue(&mut self) -> Self::Output;

    fn visit_break(&mut self) -> Self::Output;

    fn visit_for(
        &mut self,
        initializer: &Option<Box<StmtKind>>,
        condition: &Option<Expr>,
        increment: &Option<Expr>,
        body: &StmtKind,
    ) -> Self::Output;
}

/// Default walking algorithm for statements
pub fn walk_stmt<V: StmtVisitor>(stmt: &StmtKind, visitor: &mut V) -> V::Output {
    match stmt {
        StmtKind::Block(stmts) => visitor.visit_block(stmts),
        StmtKind::Expression(expr) => visitor.visit_expression(expr),
        StmtKind::Print(expr) => visitor.visit_print(expr),
        StmtKind::Variable { name, initializer } => visitor.visit_variable(name, initializer),
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
