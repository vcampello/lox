use std::{iter::Peekable, slice::Iter};

use crate::{
    ast::expression::{Expr, LiteralValue},
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    tokens: &'a [Token],
    iter: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            iter: tokens.iter().peekable(),
        }
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        matches!(self.iter.peek(), Some(t) if t.token_type == *token_type)
    }

    fn match_tokens(&mut self, token_types: &[TokenType]) -> Option<&Token> {
        for token_type in token_types.iter() {
            if self.check(token_type) {
                return self.iter.next();
            }
        }

        None
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while let Some(token) = self.match_tokens(&[TokenType::Equal, TokenType::EqualEqual]) {
            let operator = token.clone();
            let right = self.comparison();
            expr = Expr::new_binary(expr, operator, right)
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while let Some(token) = self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = token.clone();
            let right = self.term();
            expr = Expr::new_binary(expr, operator, right)
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while let Some(token) = self.match_tokens(&[TokenType::Plus, TokenType::Minus]) {
            let operator = token.clone();
            let right = self.term();
            expr = Expr::new_binary(expr, operator, right)
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while let Some(token) = self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = token.clone();
            let right = self.term();
            expr = Expr::new_binary(expr, operator, right)
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        match self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            Some(token) => {
                let operator = token.clone();
                let right = self.term();

                Expr::new_unary(operator, right)
            }
            None => self.primary(),
        }
    }

    fn primary(&mut self) -> Expr {
        if self.match_tokens(&[TokenType::False]).is_some() {
            return Expr::Literal(LiteralValue::Bool(false));
        }

        if self.match_tokens(&[TokenType::True]).is_some() {
            return Expr::Literal(LiteralValue::Bool(true));
        }

        if self.match_tokens(&[TokenType::Nil]).is_some() {
            return Expr::Literal(LiteralValue::Nil);
        }

        if self.match_tokens(&[TokenType::Nil]).is_some() {
            return Expr::Literal(LiteralValue::Nil);
        }

        // REVIEW: this doesn't work as intended as the enum require values. I'll need to review the TokenType
        // match self.match_tokens(&[TokenType::Number, TokenType::String]) {
        //     Some(token) => {
        //         let operator = token.clone();
        //         let right = self.term();
        //
        //         return Expr::new_unary(operator, right);
        //     }
        //     None => (),
        // };
        //

        if self.match_tokens(&[TokenType::LeftParen]).is_some() {
            // TODO: use self.consume
            // consume(RIGHT_PAREN, "Expect ')' after expression.");
            return Expr::new_grouping(self.expression());
        }

        // TODO: proper error handling
        todo!("finish implementing Parser.primary");
    }

    pub fn parse(&mut self) -> Expr {
        todo!();
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
