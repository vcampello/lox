use crate::ast::Expr;
use crate::frontend::Token;

#[derive(Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Expr),
    Print(Expr),
    Var {
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
    For {
        initializer: Option<Box<Stmt>>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Box<Stmt>,
    },
}

impl Stmt {
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
