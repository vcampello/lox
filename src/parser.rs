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

    fn equality(&mut self) -> ParserResult<Expr> {
        let mut expr = self.comparison()?;

        while let Some(token) = self.match_tokens(&[TokenType::Equal, TokenType::EqualEqual]) {
            let operator = token;
            let right = self.comparison()?;
            expr = Expr::new_binary(expr, operator, right)
        }

        Ok(expr)
    }

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

    fn term(&mut self) -> ParserResult<Expr> {
        let mut expr = self.factor()?;

        while let Some(token) = self.match_tokens(&[TokenType::Plus, TokenType::Minus]) {
            let operator = token;
            let right = self.term()?;
            expr = Expr::new_binary(expr, operator, right)
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ParserResult<Expr> {
        let mut expr = self.unary()?;

        while let Some(token) = self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = token;
            let right = self.term()?;
            expr = Expr::new_binary(expr, operator, right)
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ParserResult<Expr> {
        match self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            Some(token) => {
                let operator = token;
                let right = self.term()?;

                Ok(Expr::new_unary(operator, right))
            }
            None => self.primary(),
        }
    }

    fn primary(&mut self) -> ParserResult<Expr> {
        if self.match_tokens(&[TokenType::False]).is_some() {
            return Ok(Expr::Literal(LiteralValue::Bool(false)));
        }

        if self.match_tokens(&[TokenType::True]).is_some() {
            return Ok(Expr::Literal(LiteralValue::Bool(true)));
        }

        if self.match_tokens(&[TokenType::Nil]).is_some() {
            return Ok(Expr::Literal(LiteralValue::Nil));
        }

        // REVIEW: this doesn't work as intended as the enum require values. I'll need to review the TokenType
        if let Some(token) =
            self.match_tokens(&[TokenType::Number(0.0), TokenType::String(String::new())])
        {
            let operator = token;
            let right = self.term()?;
            return Ok(Expr::new_unary(operator, right));
        }

        // handle grouping expression
        if self.match_tokens(&[TokenType::LeftParen]).is_some() {
            self.consume(TokenType::RightParen, "Expected ')' after expression.")?;
            return Ok(Expr::new_grouping(self.expression()?));
        }

        eprintln!("this is not implemented properly.");
        // dbg!(&self.iter);
        // FIX: this should return the toke in self.iter.peek()
        Err(ParserError::ExpectedExpression)
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> ParserResult<&Token> {
        if !self.check(&token_type) {
            self.iter.next();
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
