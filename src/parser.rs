use std::{iter::Peekable, slice::Iter};

use crate::{
    ast::expression::Expr,
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

    fn comparison(&self) -> Expr {
        todo!()
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
