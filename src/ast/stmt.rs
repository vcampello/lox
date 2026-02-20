use crate::ast::expr::Expr;
use crate::frontend::token::Token;

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
}

impl Stmt {
    pub fn new_conditional(condition: Expr, when_true: Stmt, when_false: Option<Stmt>) -> Self {
        Self::Conditional {
            condition,
            when_true: Box::new(when_true),
            when_false: when_false.map(Box::new),
        }
    }
}
