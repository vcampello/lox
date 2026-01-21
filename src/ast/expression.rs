use crate::token::Token;

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
    Conditional {
        condition: Box<Expr>,
        when_true: Box<Expr>,
        when_false: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Variable {
        name: Token,
    },
    Assignment {
        name: Token,
        value: Box<Expr>,
    },

    // Treat literals as individual expressions
    BoolLiteral(bool),
    NumberLiteral(f64),
    StringLiteral(String),
    Nil,
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

    pub fn new_conditional(condition: Expr, when_true: Expr, when_false: Expr) -> Expr {
        Self::Conditional {
            condition: Box::new(condition),
            when_true: Box::new(when_true),
            when_false: Box::new(when_false),
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
            Expr::Conditional {
                condition,
                when_true,
                when_false,
            } => {
                format!(
                    "(conditional {} {} {})",
                    Expr::print(condition),
                    Expr::print(when_true),
                    Expr::print(when_false)
                )
            }
            Expr::Assignment { name, value } => format!("{} {}", name.lexeme, Expr::print(value)),

            Expr::StringLiteral(value) => value.clone(),
            Expr::NumberLiteral(value) => {
                // REVIEW: could this simply be "value.to_string()"?
                format!("{value}")
            }
            Expr::BoolLiteral(value) => {
                format!("{value}")
            }
            Expr::Nil => "nil".to_string(),
            Expr::Grouping(e) => {
                format!("(group {})", Expr::print(e))
            }
            Expr::Variable { name } => name.lexeme.to_string(),
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
        let literal = Expr::NumberLiteral(1.0);
        let e = Expr::new_unary(operator, literal);
        let result = Expr::print(&e);
        assert_eq!(result, "(- 1)")
    }

    #[test]
    fn binary() {
        let operator = Token::new(TokenType::Minus, String::from("-"), 1);
        let literal = Expr::NumberLiteral(1.0);
        let e = Expr::new_binary(literal.clone(), operator, literal);
        let result = Expr::print(&e);
        assert_eq!(result, "(- 1 1)")
    }

    #[test]
    fn literal() {
        let literal = Expr::NumberLiteral(1.0);
        let result = Expr::print(&literal);
        assert_eq!(result, "1")
    }

    #[test]
    fn grouping() {
        let literal = Expr::NumberLiteral(1.0);
        let e = Expr::new_grouping(literal);
        let result = Expr::print(&e);
        assert_eq!(result, "(group 1)")
    }

    #[test]
    fn nested() {
        let left = Expr::new_unary(
            Token::new(TokenType::Minus, "-".to_string(), 1),
            Expr::NumberLiteral(123.0),
        );
        let right = Expr::new_grouping(Expr::NumberLiteral(45.67));

        let operator = Token::new(TokenType::Star, "*".to_string(), 1);

        let e = Expr::new_binary(left, operator, right);
        let result = Expr::print(&e);
        assert_eq!(result, "(* (- 123) (group 45.67))")
    }
}
