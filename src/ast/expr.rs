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
    Literal {
        // This cannot be Token because it needs to be a subset which has literal values
        value: LiteralValue,
    },
    Grouping {
        expr: Box<Expr>,
    },
}

// REVIEW: this could be a trait and then there could be an AST printer
impl Expr {
    // NOTE: I'll see how far I can get without the visitor pattern suggested in the book
    fn print(e: &Expr) -> String {
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
            Expr::Literal { value } => {
                format!("{value}")
            }
            Expr::Grouping { expr } => {
                format!("(group {})", Expr::print(expr))
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
        let literal = Expr::Literal {
            value: LiteralValue::Number(1.0),
        };
        let e = Expr::Unary {
            operator,
            right: Box::new(literal),
        };
        let result = Expr::print(&e);
        assert_eq!(result, "(- 1)")
    }

    #[test]
    fn binary() {
        let operator = Token::new(TokenType::Minus, String::from("-"), 1);
        let literal = Expr::Literal {
            value: LiteralValue::Number(1.0),
        };
        let e = Expr::Binary {
            left: Box::new(literal.clone()),
            operator,
            right: Box::new(literal),
        };
        let result = Expr::print(&e);
        assert_eq!(result, "(- 1 1)")
    }

    #[test]
    fn literal() {
        let literal = Expr::Literal {
            value: LiteralValue::Number(1.0),
        };
        let result = Expr::print(&literal);
        assert_eq!(result, "1")
    }

    #[test]
    fn grouping() {
        let literal = Expr::Literal {
            value: LiteralValue::Number(1.0),
        };
        let e = Expr::Grouping {
            expr: Box::new(literal),
        };
        let result = Expr::print(&e);
        assert_eq!(result, "(group 1)")
    }

    #[test]
    fn nested() {
        let left = Box::new(Expr::Unary {
            operator: Token::new(TokenType::Minus, "-".to_string(), 1),
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(123.0),
            }),
        });
        let right = Box::new(Expr::Grouping {
            expr: Box::new(Expr::Literal {
                value: LiteralValue::Number(45.67),
            }),
        });

        let operator = Token::new(TokenType::Star, "*".to_string(), 1);

        let e = Expr::Binary {
            left,
            operator,
            right,
        };
        let result = Expr::print(&e);
        assert_eq!(result, "(* (- 123) (group 45.67))")
    }
}
