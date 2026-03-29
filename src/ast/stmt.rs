use crate::ast::{AstPrinter, Expr};
use crate::frontend::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Expr),
    Print(Expr),
    Variable {
        name: Token,
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

impl Stmt {
    pub fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::StmtOutput {
        walk_stmt(self, visitor)
    }

    pub fn new_conditional(condition: Expr, when_true: Stmt, when_false: Option<Stmt>) -> Self {
        Self::Conditional {
            condition,
            when_true: Box::new(when_true),
            when_false: when_false.map(Box::new),
        }
    }

    pub fn new_while(condition: Expr, body: Stmt) -> Self {
        Self::While {
            condition,
            body: Box::new(body),
        }
    }

    pub fn new_for(
        initializer: Option<Stmt>,
        condition: Option<Expr>,
        increment: Option<Expr>,
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
    type StmtOutput;

    fn visit_block(&mut self, stmts: &[Stmt]) -> Self::StmtOutput;

    fn visit_expression(&mut self, expr: &Expr) -> Self::StmtOutput;

    fn visit_print(&mut self, expr: &Expr) -> Self::StmtOutput;

    fn visit_variable(&mut self, name: &Token, initializer: &Option<Expr>) -> Self::StmtOutput;

    fn visit_conditional(
        &mut self,
        condition: &Expr,
        when_true: &Stmt,
        when_false: &Option<Box<Stmt>>,
    ) -> Self::StmtOutput;

    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> Self::StmtOutput;

    fn visit_continue(&mut self) -> Self::StmtOutput;

    fn visit_break(&mut self) -> Self::StmtOutput;

    fn visit_for(
        &mut self,
        initializer: &Option<Box<Stmt>>,
        condition: &Option<Expr>,
        increment: &Option<Expr>,
        body: &Stmt,
    ) -> Self::StmtOutput;
}

/// Default walking algorithm for statements
pub fn walk_stmt<V: StmtVisitor>(stmt: &Stmt, visitor: &mut V) -> V::StmtOutput {
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
