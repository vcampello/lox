use core::fmt;
use std::{iter::Peekable, slice::Iter};

use crate::{
    ast::expression::{Expr, LiteralValue},
    token::{Token, TokenType},
};

#[derive(Debug, Clone)]
pub enum ParserError {
    ExpectedToken(TokenType),
    ExpectedExpression,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectedToken(v) => write!(f, "Expected to find '{v}', after expression."),
            Self::ExpectedExpression => write!(f, "Expected expression."),
        }
    }
}

pub type ParserResult<T> = Result<T, ParserError>;

pub struct Parser<'a> {
    iter: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            iter: tokens.iter().peekable(),
        }
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        matches!(self.iter.peek(), Some(t) if t.token_type == *token_type)
    }

    fn match_tokens(&mut self, token_types: &[TokenType]) -> Option<Token> {
        for token_type in token_types.iter() {
            if self.check(token_type) {
                return dbg!(self.iter.next().cloned());
            }
        }

        None
    }

    fn expression(&mut self) -> ParserResult<Expr> {
        self.equality()
    }

    /// equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> ParserResult<Expr> {
        let mut expr = self.comparison()?;

        while let Some(token) = self.match_tokens(&[TokenType::Equal, TokenType::EqualEqual]) {
            let operator = token;
            let right = self.comparison()?;
            expr = Expr::new_binary(expr, operator, right)
        }

        Ok(expr)
    }

    /// comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> ParserResult<Expr> {
        let mut expr = self.term()?;

        while let Some(token) = self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = token;
            let right = self.term()?;
            expr = Expr::new_binary(expr, operator, right)
        }

        Ok(expr)
    }

    /// term → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> ParserResult<Expr> {
        let mut expr = self.factor()?;

        while let Some(token) = self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = token;
            let right = self.factor()?;
            expr = Expr::new_binary(expr, operator, right)
        }

        Ok(expr)
    }

    /// factor → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> ParserResult<Expr> {
        let mut expr = self.unary()?;

        while let Some(token) = self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = token;
            let right = self.unary()?;
            expr = Expr::new_binary(expr, operator, right)
        }

        Ok(expr)
    }

    /// unary → ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> ParserResult<Expr> {
        match self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            Some(token) => {
                let operator = token;
                let right = self.unary()?;

                Ok(Expr::new_unary(operator, right))
            }
            None => self.primary(),
        }
    }

    /// primary → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> ParserResult<Expr> {
        // NOTE: we'll bypass match_tokens to make this more readable
        if let Some(token) = self.iter.next() {
            match &token.token_type {
                TokenType::True => return Ok(Expr::Literal(LiteralValue::Bool(true))),
                TokenType::False => return Ok(Expr::Literal(LiteralValue::Bool(false))),
                TokenType::Nil => return Ok(Expr::Literal(LiteralValue::Nil)),
                TokenType::Number(v) => return Ok(Expr::Literal(LiteralValue::Number(*v))),
                TokenType::String(v) => {
                    return Ok(Expr::Literal(LiteralValue::String(v.to_string())));
                }
                TokenType::LeftParen => {
                    let expr = self.expression()?; // must be called before consuming
                    self.consume(TokenType::RightParen, "Expected ')' after expression.")?;
                    return Ok(Expr::new_grouping(expr));
                }

                _ => (), // just let it fall-through
            };
        }

        Err(ParserError::ExpectedExpression)
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> ParserResult<&Token> {
        if !self.check(&token_type) {
            // self.iter.next();
            return Err(ParserError::ExpectedToken(token_type));
        }

        // TODO: improve this. Maybe some kind of map?
        // We know it's the right token because of self.check
        Ok(self
            .iter
            .next()
            .expect("Expected to find matching TokenType"))
    }

    // FIXME: does this error need to cross this module's boundary?
    pub fn parse(&mut self) -> ParserResult<Expr> {
        self.expression()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn lifetime() {
        let tokens: Vec<Token> = Vec::new();
        let _parser = Parser::new(&tokens);
        todo!();
    }
}
