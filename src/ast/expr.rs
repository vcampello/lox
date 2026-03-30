use crate::{ast::AstPrinter, frontend::Token};

#[derive(Debug, Clone)]
pub enum Expr {
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
        name: Token,
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

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut printer = AstPrinter::new();
        write!(f, "{}", self.accept(&mut printer))
    }
}

impl Expr {
    pub fn new_unary(operator: Token, right: Expr) -> Expr {
        Self::Unary {
            operator,
            right: Box::new(right),
        }
    }

    pub fn new_binary(left: Expr, operator: Token, right: Expr) -> Expr {
        Self::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn new_grouping(expr: Expr) -> Expr {
        Self::Grouping(Box::new(expr))
    }

    pub fn new_assignment(name: Token, value: Expr) -> Expr {
        Self::Assignment {
            name,
            value: Box::new(value),
        }
    }

    pub fn new_logical(left: Expr, operator: Token, right: Expr) -> Expr {
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

    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Self::Output;

    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Self::Output;

    fn visit_grouping(&mut self, expr: &Expr) -> Self::Output;

    fn visit_variable(&mut self, name: &Token) -> Self::Output;

    fn visit_assignment(&mut self, name: &Token, value: &Expr) -> Self::Output;

    fn visit_logical(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Self::Output;

    fn visit_bool(&mut self, value: &bool) -> Self::Output;

    fn visit_number(&mut self, value: &f64) -> Self::Output;

    fn visit_string(&mut self, value: &str) -> Self::Output;

    fn visit_nil(&mut self) -> Self::Output;
}

/// Default walking algorithm for expressions
pub fn walk_expr<V: ExprVisitor>(expr: &Expr, visitor: &mut V) -> V::Output {
    match expr {
        Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
        Expr::Binary {
            left,
            operator,
            right,
        } => visitor.visit_binary(left, operator, right),
        Expr::Grouping(expr) => visitor.visit_grouping(expr),
        Expr::Variable { name } => visitor.visit_variable(name),
        Expr::Assignment { name, value } => visitor.visit_assignment(name, value),
        Expr::Logical {
            left,
            operator,
            right,
        } => visitor.visit_logical(left, operator, right),
        Expr::BoolLiteral(v) => visitor.visit_bool(v),
        Expr::NumberLiteral(v) => visitor.visit_number(v),
        Expr::StringLiteral(v) => visitor.visit_string(v),
        Expr::Nil => visitor.visit_nil(),
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
        let literal = Expr::NumberLiteral(1.0);
        let e = Expr::new_unary(operator, literal);
        let mut printer = AstPrinter::new();
        let result = e.accept(&mut printer);
        assert_eq!(result, "(- 1)")
    }

    #[test]
    fn binary() {
        let operator = Token::new(TokenKind::Minus, String::from("-"), Span::new(1, 1, (0, 1)));
        let literal = Expr::NumberLiteral(1.0);
        let e = Expr::new_binary(literal.clone(), operator, literal);
        let mut printer = AstPrinter::new();
        let result = e.accept(&mut printer);
        assert_eq!(result, "(- 1 1)")
    }

    #[test]
    fn literal() {
        let literal = Expr::NumberLiteral(1.0);
        let mut printer = AstPrinter::new();
        let result = literal.accept(&mut printer);
        assert_eq!(result, "1")
    }

    #[test]
    fn grouping() {
        let literal = Expr::NumberLiteral(1.0);
        let e = Expr::new_grouping(literal);
        let mut printer = AstPrinter::new();
        let result = e.accept(&mut printer);
        assert_eq!(result, "(group 1)")
    }

    #[test]
    fn nested() {
        let left = Expr::new_unary(
            Token::new(TokenKind::Minus, "-".to_string(), Span::new(1, 1, (0, 1))),
            Expr::NumberLiteral(123.0),
        );
        let right = Expr::new_grouping(Expr::NumberLiteral(45.67));
        let operator = Token::new(TokenKind::Star, "*".to_string(), Span::new(1, 1, (0, 1)));

        let e = Expr::new_binary(left, operator, right);
        let mut printer = AstPrinter::new();
        let result = e.accept(&mut printer);
        assert_eq!(result, "(* (- 123) (group 45.67))")
    }
}
