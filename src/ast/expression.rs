use std::fmt;

use crate::token::Token;

#[derive(Debug, Clone)]
pub enum LiteralValue {
    String(String),
    Number(f64),
    Bool(bool),
    Nil, // REVIEW: should this be Nil(None)?
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Literals.
            LiteralValue::String(v) => write!(f, "{v}"),
            LiteralValue::Number(v) => write!(f, "{v}"),
            LiteralValue::Bool(v) => write!(f, "{v}"),
            LiteralValue::Nil => write!(f, "nil"),
        }
    }
}

// REVIEW: expr::Expr is confusing. Is this the correct way to handle this?
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
    // This cannot be Token because it needs to be a subset which has literal values
    Literal(LiteralValue),
    Grouping(Box<Expr>),
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

    // REVIEW: this could be a trait and then there could be an AST printer
    // NOTE: I'll see how far I can get without the visitor pattern suggested in the book
    pub fn print(e: &Expr) -> String {
        match e {
            Expr::Unary { operator, right } => {
                format!("({} {})", operator.lexeme, Expr::print(right))
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                format!(
                    "({} {} {})",
                    operator.lexeme,
                    Expr::print(left),
                    Expr::print(right)
                )
            }
            Expr::Literal(value) => {
                format!("{value}")
            }
            Expr::Grouping(e) => {
                format!("(group {})", Expr::print(e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    #[test]
    fn unary() {
        let operator = Token::new(TokenType::Minus, String::from("-"), 1);
        let literal = Expr::Literal(LiteralValue::Number(1.0));
        let e = Expr::new_unary(operator, literal);
        let result = Expr::print(&e);
        assert_eq!(result, "(- 1)")
    }

    #[test]
    fn binary() {
        let operator = Token::new(TokenType::Minus, String::from("-"), 1);
        let literal = Expr::Literal(LiteralValue::Number(1.0));
        let e = Expr::new_binary(literal.clone(), operator, literal);
        let result = Expr::print(&e);
        assert_eq!(result, "(- 1 1)")
    }

    #[test]
    fn literal() {
        let literal = Expr::Literal(LiteralValue::Number(1.0));
        let result = Expr::print(&literal);
        assert_eq!(result, "1")
    }

    #[test]
    fn grouping() {
        let literal = Expr::Literal(LiteralValue::Number(1.0));
        let e = Expr::new_grouping(literal);
        let result = Expr::print(&e);
        assert_eq!(result, "(group 1)")
    }

    #[test]
    fn nested() {
        let left = Expr::new_unary(
            Token::new(TokenType::Minus, "-".to_string(), 1),
            Expr::Literal(LiteralValue::Number(123.0)),
        );
        let right = Expr::new_grouping(Expr::Literal(LiteralValue::Number(45.67)));

        let operator = Token::new(TokenType::Star, "*".to_string(), 1);

        let e = Expr::new_binary(left, operator, right);
        let result = Expr::print(&e);
        assert_eq!(result, "(* (- 123) (group 45.67))")
    }
}
