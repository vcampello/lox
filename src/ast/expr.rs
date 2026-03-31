use crate::{ast::AstPrinter, common::Span, frontend::Token};

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn unary(operator: Token, right: Expr) -> Self {
        Self {
            span: operator.span.merge(&right.span),
            kind: ExprKind::Unary {
                operator,
                right: Box::new(right),
            },
        }
    }

    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            span: left.span.merge(&right.span),
            kind: ExprKind::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            },
        }
    }

    pub fn grouping(expr: Expr) -> Self {
        Self {
            span: expr.span,
            kind: ExprKind::Grouping(Box::new(expr)),
        }
    }

    pub fn assignment(name: Token, value: Expr) -> Self {
        Self {
            span: name.span.merge(&value.span),
            kind: ExprKind::Assignment {
                name,
                value: Box::new(value),
            },
        }
    }

    pub fn logical(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            span: left.span.merge(&right.span),
            kind: ExprKind::Logical {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            },
        }
    }

    pub fn variable(token: Token, at: Span) -> Self {
        Self {
            span: at,
            kind: ExprKind::Variable { var: token },
        }
    }

    pub fn bool_literal(value: bool, at: Span) -> Self {
        Self {
            span: at,
            kind: ExprKind::BoolLiteral(value),
        }
    }

    pub fn number_literal(value: f64, at: Span) -> Self {
        Self {
            span: at,
            kind: ExprKind::NumberLiteral(value),
        }
    }

    pub fn string_literal(value: String, at: Span) -> Self {
        Self {
            span: at,
            kind: ExprKind::StringLiteral(value),
        }
    }

    pub fn nil(at: Span) -> Self {
        Self {
            span: at,
            kind: ExprKind::Nil,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Variable {
        var: Token,
    },
    Assignment {
        name: Token,
        value: Box<Expr>,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },

    // Treat literals as individual expressions
    BoolLiteral(bool),
    NumberLiteral(f64),
    StringLiteral(String),
    Nil,
}

impl std::fmt::Display for ExprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut printer = AstPrinter::new();
        write!(f, "{}", self.accept(&mut printer))
    }
}

impl ExprKind {
    pub fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output {
        walk_expr(self, visitor)
    }
}

pub trait ExprVisitor {
    type Output;

    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Self::Output;

    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Self::Output;

    fn visit_grouping(&mut self, expr: &Expr) -> Self::Output;

    fn visit_variable(&mut self, var: &Token) -> Self::Output;

    fn visit_assignment(&mut self, name: &Token, value: &Expr) -> Self::Output;

    fn visit_logical(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Self::Output;

    fn visit_bool(&mut self, value: &bool) -> Self::Output;

    fn visit_number(&mut self, value: &f64) -> Self::Output;

    fn visit_string(&mut self, value: &str) -> Self::Output;

    fn visit_nil(&mut self) -> Self::Output;
}

/// Default walking algorithm for expressions
pub fn walk_expr<V: ExprVisitor>(expr: &ExprKind, visitor: &mut V) -> V::Output {
    match expr {
        ExprKind::Unary { operator, right } => visitor.visit_unary(operator, right),
        ExprKind::Binary {
            left,
            operator,
            right,
        } => visitor.visit_binary(left, operator, right),
        ExprKind::Grouping(expr) => visitor.visit_grouping(expr),
        ExprKind::Variable { var: name } => visitor.visit_variable(name),
        ExprKind::Assignment { name, value } => visitor.visit_assignment(name, value),
        ExprKind::Logical {
            left,
            operator,
            right,
        } => visitor.visit_logical(left, operator, right),
        ExprKind::BoolLiteral(v) => visitor.visit_bool(v),
        ExprKind::NumberLiteral(v) => visitor.visit_number(v),
        ExprKind::StringLiteral(v) => visitor.visit_string(v),
        ExprKind::Nil => visitor.visit_nil(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::AstPrinter,
        common::Span,
        frontend::{Token, TokenKind},
    };

    #[test]
    fn unary() {
        let operator = Token::new(TokenKind::Minus, String::from("-"), Span::default());
        let literal = Expr::number_literal(1.0, Span::default());
        let e = Expr::unary(operator, literal);
        let mut printer = AstPrinter::new();
        let result = e.kind.accept(&mut printer);
        assert_eq!(result, "(- 1)")
    }

    #[test]
    fn binary() {
        let operator = Token::new(TokenKind::Minus, String::from("-"), Span::default());
        let literal = Expr::number_literal(1.0, Span::default());
        let e = Expr::binary(literal.clone(), operator, literal);
        let mut printer = AstPrinter::new();
        let result = e.kind.accept(&mut printer);
        assert_eq!(result, "(- 1 1)")
    }

    #[test]
    fn literal() {
        let literal = ExprKind::NumberLiteral(1.0);
        let mut printer = AstPrinter::new();
        let result = literal.accept(&mut printer);
        assert_eq!(result, "1")
    }

    #[test]
    fn grouping() {
        let literal = Expr::number_literal(1.0, Span::default());
        let e = Expr::grouping(literal);
        let mut printer = AstPrinter::new();
        let result = e.kind.accept(&mut printer);
        assert_eq!(result, "(group 1)")
    }

    #[test]
    fn nested() {
        let left = Expr::unary(
            Token::new(TokenKind::Minus, "-".to_string(), Span::default()),
            Expr::number_literal(123.0, Span::default()),
        );
        let right = Expr::grouping(Expr::number_literal(45.67, Span::default()));
        let operator = Token::new(TokenKind::Star, "*".to_string(), Span::default());

        let e = Expr::binary(left, operator, right);
        let mut printer = AstPrinter::new();
        let result = e.kind.accept(&mut printer);
        assert_eq!(result, "(* (- 123) (group 45.67))")
    }
}
