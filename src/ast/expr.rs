use std::ops::Deref;

use super::{Token, TokenKind};
use crate::{
    ast::AstPrinter,
    common::{Span, Spanned},
    frontend::ParserErrorKind,
};

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

// REVIEW: From<ExprKind> for Expr may be a better fit
impl Expr {
    pub fn grouping(expr: Expr) -> Self {
        GroupingExpr::new(expr).into()
    }

    pub fn assignment(name: Token, value: Expr) -> Self {
        AssignmentExpr::new(name, value).into()
    }

    pub fn logical(left: Expr, operator: Spanned<LogicalOp>, right: Expr) -> Self {
        LogicalExpr::new(left, operator, right).into()
    }

    pub fn variable(token: Token) -> Self {
        VariableExpr::new(token).into()
    }

    pub fn bool_literal(value: bool, at: Span) -> Self {
        BoolLiteralExpr::new(value, at).into()
    }

    pub fn number_literal(value: f64, at: Span) -> Self {
        NumberLiteralExpr::new(value, at).into()
    }

    pub fn string_literal(value: String, at: Span) -> Self {
        StringLiteralExpr::new(value, at).into()
    }

    pub fn nil(at: Span) -> Self {
        NilExpr::new(at).into()
    }

    pub fn unary(operator: Spanned<UnaryOp>, right: Expr) -> Self {
        UnaryExpr::new(operator, right).into()
    }

    pub fn binary(left: Expr, operator: Spanned<BinaryOp>, right: Expr) -> Self {
        BinaryExpr::new(left, operator, right).into()
    }
}

impl From<ExprKind> for Expr {
    fn from(value: ExprKind) -> Self {
        match value {
            ExprKind::Unary(unary_expr) => unary_expr.into(),
            ExprKind::Binary(binary_expr) => binary_expr.into(),
            ExprKind::Grouping(grouping_expr) => grouping_expr.into(),
            ExprKind::Variable(variable_expr) => variable_expr.into(),
            ExprKind::Assignment(assignment_expr) => assignment_expr.into(),
            ExprKind::Logical(logical_expr) => logical_expr.into(),
            ExprKind::BoolLiteral(bool_literal_expr) => bool_literal_expr.into(),
            ExprKind::NumberLiteral(number_literal_expr) => number_literal_expr.into(),
            ExprKind::StringLiteral(string_literal_expr) => string_literal_expr.into(),
            ExprKind::Nil(nil_expr) => nil_expr.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    /// Negative
    Neg,
    Not,
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Not => write!(f, "!"),
        }
    }
}

impl TryFrom<TokenKind> for UnaryOp {
    type Error = ParserErrorKind;

    fn try_from(token_kind: TokenKind) -> Result<Self, Self::Error> {
        match token_kind {
            TokenKind::Bang => Ok(UnaryOp::Not),
            TokenKind::Minus => Ok(UnaryOp::Neg),
            _ => Err(ParserErrorKind::InvalidOperator {
                operation: "unary",
                token_kind,
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    EqualEqual,
    BangEqual,
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BinaryOp::*;
        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/"),
            Greater => write!(f, ">"),
            GreaterEqual => write!(f, ">="),
            Less => write!(f, "<"),
            LessEqual => write!(f, "<="),
            EqualEqual => write!(f, "=="),
            BangEqual => write!(f, "!="),
        }
    }
}

impl TryFrom<TokenKind> for BinaryOp {
    type Error = ParserErrorKind;

    fn try_from(kind: TokenKind) -> Result<Self, Self::Error> {
        use BinaryOp::*;
        match kind {
            TokenKind::Plus => Ok(Add),
            TokenKind::Minus => Ok(Sub),
            TokenKind::Star => Ok(Mul),
            TokenKind::Slash => Ok(Div),
            TokenKind::Greater => Ok(Greater),
            TokenKind::GreaterEqual => Ok(GreaterEqual),
            TokenKind::Less => Ok(Less),
            TokenKind::LessEqual => Ok(LessEqual),
            TokenKind::EqualEqual => Ok(EqualEqual),
            TokenKind::BangEqual => Ok(BangEqual),
            _ => Err(ParserErrorKind::InvalidOperator {
                operation: "binary",
                token_kind: kind,
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LogicalOp {
    And,
    Or,
}

impl std::fmt::Display for LogicalOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicalOp::And => write!(f, "and"),
            LogicalOp::Or => write!(f, "or"),
        }
    }
}

impl TryFrom<TokenKind> for LogicalOp {
    type Error = ParserErrorKind;

    fn try_from(kind: TokenKind) -> Result<Self, Self::Error> {
        match kind {
            TokenKind::And => Ok(LogicalOp::And),
            TokenKind::Or => Ok(LogicalOp::Or),
            _ => Err(ParserErrorKind::InvalidOperator {
                operation: "logical",
                token_kind: kind,
            }),
        }
    }
}

impl Deref for Expr {
    type Target = ExprKind;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Variable(VariableExpr),
    Assignment(AssignmentExpr),
    Logical(LogicalExpr),

    // Treat literals as individual expressions
    BoolLiteral(BoolLiteralExpr),
    NumberLiteral(NumberLiteralExpr),
    StringLiteral(StringLiteralExpr),
    Nil(NilExpr),
}

impl std::fmt::Display for ExprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut printer = AstPrinter::new();
        write!(f, "{}", self.visit(&mut printer))
    }
}

impl ExprKind {
    pub fn visit<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output {
        walk_expr(self, visitor)
    }
}

pub trait ExprVisitor {
    type Output;

    /// Defines the expression evaluation algorithm
    fn visit_expr(&mut self, expr: &Expr) -> Self::Output
    where
        Self: Sized,
    {
        walk_expr(&expr.kind, self)
    }

    fn visit_unary(&mut self, expr: &UnaryExpr) -> Self::Output;

    fn visit_binary(&mut self, expr: &BinaryExpr) -> Self::Output;

    fn visit_grouping(&mut self, expr: &GroupingExpr) -> Self::Output;

    fn visit_variable(&mut self, expr: &VariableExpr) -> Self::Output;

    fn visit_assignment(&mut self, expr: &AssignmentExpr) -> Self::Output;

    fn visit_logical(&mut self, expr: &LogicalExpr) -> Self::Output;

    fn visit_bool(&mut self, expr: &BoolLiteralExpr) -> Self::Output;

    fn visit_number(&mut self, expr: &NumberLiteralExpr) -> Self::Output;

    fn visit_string(&mut self, expr: &StringLiteralExpr) -> Self::Output;

    fn visit_nil(&mut self, expr: &NilExpr) -> Self::Output;
}

/// Default walking algorithm for expressions
pub fn walk_expr<V: ExprVisitor>(expr: &ExprKind, visitor: &mut V) -> V::Output {
    match expr {
        ExprKind::Unary(e) => visitor.visit_unary(e),
        ExprKind::Binary(e) => visitor.visit_binary(e),
        ExprKind::Grouping(e) => visitor.visit_grouping(e),
        ExprKind::Variable(e) => visitor.visit_variable(e),
        ExprKind::Assignment(e) => visitor.visit_assignment(e),
        ExprKind::Logical(e) => visitor.visit_logical(e),
        ExprKind::BoolLiteral(e) => visitor.visit_bool(e),
        ExprKind::NumberLiteral(e) => visitor.visit_number(e),
        ExprKind::StringLiteral(e) => visitor.visit_string(e),
        ExprKind::Nil(e) => visitor.visit_nil(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::AstPrinter, common::Span};

    #[test]
    fn unary() {
        let operator = Token::new(TokenKind::Minus, String::from("-"), Span::default());
        let op = UnaryOp::try_from(operator.kind).unwrap();
        let literal = Expr::number_literal(1.0, Span::default());
        let e = Expr::unary(Spanned::new(op, operator.span), literal);
        let mut printer = AstPrinter::new();
        let result = e.visit(&mut printer);
        assert_eq!(result, "(- 1)")
    }

    #[test]
    fn binary() {
        let operator = Token::new(TokenKind::Minus, String::from("-"), Span::default());
        let op = BinaryOp::try_from(operator.kind).unwrap();
        let literal = Expr::number_literal(1.0, Span::default());
        let e = Expr::binary(literal.clone(), Spanned::new(op, operator.span), literal);
        let mut printer = AstPrinter::new();
        let result = e.visit(&mut printer);
        assert_eq!(result, "(- 1 1)")
    }

    #[test]
    fn literal() {
        let literal = Expr::number_literal(1.0, Span::default());
        let mut printer = AstPrinter::new();
        let result = literal.visit(&mut printer);
        assert_eq!(result, "1")
    }

    #[test]
    fn grouping() {
        let literal = Expr::number_literal(1.0, Span::default());
        let e = Expr::grouping(literal);
        let mut printer = AstPrinter::new();
        let result = e.visit(&mut printer);
        assert_eq!(result, "(group 1)")
    }

    #[test]
    fn nested() {
        let left_op_tok = Token::new(TokenKind::Minus, "-".to_string(), Span::default());
        let left_op = UnaryOp::try_from(left_op_tok.kind).unwrap();
        let left = Expr::unary(
            Spanned::new(left_op, left_op_tok.span),
            Expr::number_literal(123.0, Span::default()),
        );
        let right = Expr::grouping(Expr::number_literal(45.67, Span::default()));
        let op_tok = Token::new(TokenKind::Star, "*".to_string(), Span::default());
        let op = BinaryOp::try_from(op_tok.kind).unwrap();

        let e = Expr::binary(left, Spanned::new(op, op_tok.span), right);
        let mut printer = AstPrinter::new();
        let result = e.visit(&mut printer);
        assert_eq!(result, "(* (- 123) (group 45.67))")
    }
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub span: Span,
    pub operator: Spanned<UnaryOp>,
    pub right: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(operator: Spanned<UnaryOp>, right: Expr) -> Self {
        UnaryExpr {
            span: operator.span.merge(&right.span),
            operator,
            right: Box::new(right),
        }
    }
}

impl From<UnaryExpr> for Expr {
    fn from(value: UnaryExpr) -> Self {
        Self {
            span: value.span,
            kind: ExprKind::Unary(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub span: Span,
    pub left: Box<Expr>,
    pub operator: Spanned<BinaryOp>,
    pub right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Expr, operator: Spanned<BinaryOp>, right: Expr) -> Self {
        Self {
            span: left.span.merge(&right.span),
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

impl From<BinaryExpr> for Expr {
    fn from(value: BinaryExpr) -> Self {
        Self {
            span: value.span,
            kind: ExprKind::Binary(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GroupingExpr {
    pub span: Span,
    pub group: Box<Expr>,
}

impl GroupingExpr {
    pub fn new(group: Expr) -> Self {
        Self {
            span: group.span,
            group: Box::new(group),
        }
    }
}

impl From<GroupingExpr> for Expr {
    fn from(value: GroupingExpr) -> Self {
        Self {
            span: value.span,
            kind: ExprKind::Grouping(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VariableExpr {
    pub span: Span,
    pub var: Token,
}

impl VariableExpr {
    pub fn new(var: Token) -> Self {
        Self {
            span: var.span,
            var,
        }
    }
}

impl From<VariableExpr> for Expr {
    fn from(value: VariableExpr) -> Self {
        Self {
            span: value.span,
            kind: ExprKind::Variable(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AssignmentExpr {
    pub span: Span,
    pub name: Token,
    pub value: Box<Expr>,
}

impl AssignmentExpr {
    pub fn new(name: Token, value: Expr) -> Self {
        Self {
            span: name.span.merge(&value.span),
            name,
            value: Box::new(value),
        }
    }
}

impl From<AssignmentExpr> for Expr {
    fn from(value: AssignmentExpr) -> Self {
        Self {
            span: value.span,
            kind: ExprKind::Assignment(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogicalExpr {
    pub span: Span,
    pub left: Box<Expr>,
    pub operator: Spanned<LogicalOp>,
    pub right: Box<Expr>,
}

impl LogicalExpr {
    pub fn new(left: Expr, operator: Spanned<LogicalOp>, right: Expr) -> Self {
        Self {
            span: left.span.merge(&right.span),
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

impl From<LogicalExpr> for Expr {
    fn from(value: LogicalExpr) -> Self {
        Self {
            span: value.span,
            kind: ExprKind::Logical(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoolLiteralExpr {
    pub span: Span,
    pub value: bool,
}

impl BoolLiteralExpr {
    pub fn new(value: bool, span: Span) -> Self {
        Self { span, value }
    }
}

impl From<BoolLiteralExpr> for Expr {
    fn from(value: BoolLiteralExpr) -> Self {
        Self {
            span: value.span,
            kind: ExprKind::BoolLiteral(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NumberLiteralExpr {
    pub span: Span,
    pub value: f64,
}

impl NumberLiteralExpr {
    pub fn new(value: f64, span: Span) -> Self {
        Self { span, value }
    }
}

impl From<NumberLiteralExpr> for Expr {
    fn from(value: NumberLiteralExpr) -> Self {
        Self {
            span: value.span,
            kind: ExprKind::NumberLiteral(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StringLiteralExpr {
    pub span: Span,
    pub value: String,
}

impl StringLiteralExpr {
    pub fn new(value: String, span: Span) -> Self {
        Self { span, value }
    }
}

impl From<StringLiteralExpr> for Expr {
    fn from(value: StringLiteralExpr) -> Self {
        Self {
            span: value.span,
            kind: ExprKind::StringLiteral(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NilExpr {
    pub span: Span,
}

impl NilExpr {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

impl From<NilExpr> for Expr {
    fn from(value: NilExpr) -> Self {
        Self {
            span: value.span,
            kind: ExprKind::Nil(value),
        }
    }
}
