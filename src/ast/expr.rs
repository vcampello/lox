use std::fmt;

use crate::token::Token;

#[derive(Debug, Clone)]
pub enum LiteralValue {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Literals.
            LiteralValue::String(v) => write!(f, "String({v})"),
            LiteralValue::Number(v) => write!(f, "Number({v})"),
            LiteralValue::Bool(v) => write!(f, "Bool({v})"),
            LiteralValue::Nil => write!(f, "Nil"),
        }
    }
}

pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}
pub struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

pub struct LiteralExpr {
    // This cannot be Token because it needs to be a subset which has literal values
    value: LiteralValue,
}
pub struct GroupingExpr {
    expr: Box<Expr>,
}

pub trait Visitor<T> {
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> T;
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> T;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> T;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> T;
}

pub enum Expr {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Literal(LiteralExpr),
    Grouping(GroupingExpr),
}

impl Expr {
    fn accept<V: Visitor<Self>>(&self, visitor: &V) -> Self {
        match self {
            Expr::Unary(e) => visitor.visit_unary_expr(e),
            Expr::Binary(e) => visitor.visit_binary_expr(e),
            Expr::Grouping(e) => visitor.visit_grouping_expr(e),
            Expr::Literal(e) => visitor.visit_literal_expr(e),
        }
    }
}
