use crate::{ast::AstPrinter, frontend::Token};

#[derive(Debug, Clone)]
pub enum ExprKind {
    Unary {
        operator: Token,
        right: Box<ExprKind>,
    },
    Binary {
        left: Box<ExprKind>,
        operator: Token,
        right: Box<ExprKind>,
    },
    Grouping(Box<ExprKind>),
    Variable {
        name: Token,
    },
    Assignment {
        name: Token,
        value: Box<ExprKind>,
    },
    Logical {
        left: Box<ExprKind>,
        operator: Token,
        right: Box<ExprKind>,
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
    pub fn new_unary(operator: Token, right: ExprKind) -> ExprKind {
        Self::Unary {
            operator,
            right: Box::new(right),
        }
    }

    pub fn new_binary(left: ExprKind, operator: Token, right: ExprKind) -> ExprKind {
        Self::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn new_grouping(expr: ExprKind) -> ExprKind {
        Self::Grouping(Box::new(expr))
    }

    pub fn new_assignment(name: Token, value: ExprKind) -> ExprKind {
        Self::Assignment {
            name,
            value: Box::new(value),
        }
    }

    pub fn new_logical(left: ExprKind, operator: Token, right: ExprKind) -> ExprKind {
        Self::Logical {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output {
        walk_expr(self, visitor)
    }
}

pub trait ExprVisitor {
    type Output;

    fn visit_unary(&mut self, operator: &Token, right: &ExprKind) -> Self::Output;

    fn visit_binary(&mut self, left: &ExprKind, operator: &Token, right: &ExprKind)
    -> Self::Output;

    fn visit_grouping(&mut self, expr: &ExprKind) -> Self::Output;

    fn visit_variable(&mut self, name: &Token) -> Self::Output;

    fn visit_assignment(&mut self, name: &Token, value: &ExprKind) -> Self::Output;

    fn visit_logical(
        &mut self,
        left: &ExprKind,
        operator: &Token,
        right: &ExprKind,
    ) -> Self::Output;

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
        ExprKind::Variable { name } => visitor.visit_variable(name),
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
        let operator = Token::new(TokenKind::Minus, String::from("-"), Span::new(1, 1, (0, 1)));
        let literal = ExprKind::NumberLiteral(1.0);
        let e = ExprKind::new_unary(operator, literal);
        let mut printer = AstPrinter::new();
        let result = e.accept(&mut printer);
        assert_eq!(result, "(- 1)")
    }

    #[test]
    fn binary() {
        let operator = Token::new(TokenKind::Minus, String::from("-"), Span::new(1, 1, (0, 1)));
        let literal = ExprKind::NumberLiteral(1.0);
        let e = ExprKind::new_binary(literal.clone(), operator, literal);
        let mut printer = AstPrinter::new();
        let result = e.accept(&mut printer);
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
        let literal = ExprKind::NumberLiteral(1.0);
        let e = ExprKind::new_grouping(literal);
        let mut printer = AstPrinter::new();
        let result = e.accept(&mut printer);
        assert_eq!(result, "(group 1)")
    }

    #[test]
    fn nested() {
        let left = ExprKind::new_unary(
            Token::new(TokenKind::Minus, "-".to_string(), Span::new(1, 1, (0, 1))),
            ExprKind::NumberLiteral(123.0),
        );
        let right = ExprKind::new_grouping(ExprKind::NumberLiteral(45.67));
        let operator = Token::new(TokenKind::Star, "*".to_string(), Span::new(1, 1, (0, 1)));

        let e = ExprKind::new_binary(left, operator, right);
        let mut printer = AstPrinter::new();
        let result = e.accept(&mut printer);
        assert_eq!(result, "(* (- 123) (group 45.67))")
    }
}
