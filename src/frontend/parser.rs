use std::{iter::Peekable, slice::Iter};

use super::token::{Token, TokenType};
use crate::ast::expr::Expr;
use crate::ast::stmt::Stmt;
use crate::frontend::ParserError;

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

    pub fn parse(&mut self) -> ParserResult<Vec<Stmt>> {
        let mut stmts = Vec::new();

        while matches!(self.iter.peek(), Some(token) if token.token_type != TokenType::Eof) {
            match self.declaration() {
                Ok(stmt) => stmts.push(stmt),
                Err(e) => {
                    self.sychronise();
                    return Err(e);
                }
            }
        }

        Ok(stmts)
    }

    fn sychronise(&mut self) {
        while let Some(token) = self.advance() {
            // statement boundary reached reached
            if token.token_type == TokenType::Semicolon {
                return;
            }

            if let Some(next_token) = self.iter.peek() {
                match next_token.token_type {
                    TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
                    | TokenType::Eof => return,
                    _ => continue,
                };
            }
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        self.iter.next()
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        matches!(self.iter.peek(), Some(t) if t.token_type == *token_type)
    }

    fn is_eof(&mut self) -> bool {
        self.check(&TokenType::Eof)
    }

    fn match_tokens(&mut self, token_types: &[TokenType]) -> Option<Token> {
        for token_type in token_types.iter() {
            if self.check(token_type) {
                return self.advance().cloned();
            }
        }

        None
    }

    fn expression(&mut self) -> ParserResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParserResult<Expr> {
        let expr = self.equality()?;

        if let Some(equals) = self.match_tokens(&[TokenType::Equal]) {
            let value = self.assignment()?;

            return match expr {
                Expr::Variable { name } => Ok(Expr::new_assignment(name, value)),
                _ => Err(ParserError::InvalidAssignmentTarget { token: equals }),
            };
        }

        Ok(expr)
    }

    /// equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> ParserResult<Expr> {
        let mut expr = self.comparison()?;

        while let Some(token) = self.match_tokens(&[TokenType::EqualEqual, TokenType::BangEqual]) {
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
        // FIXME: is there a better way to enforce always having something?
        // NOTE: we'll bypass match_tokens to make this more readable
        if let Some(token) = self.advance() {
            return match &token.token_type {
                TokenType::True => Ok(Expr::BoolLiteral(true)),
                TokenType::False => Ok(Expr::BoolLiteral(false)),
                TokenType::Nil => Ok(Expr::Nil),
                TokenType::Number => token
                    .lexeme
                    .parse::<f64>()
                    .map_err(|_| ParserError::InvalidAssignmentTarget {
                        token: token.clone(),
                    })
                    .map(Expr::NumberLiteral),
                TokenType::String => Ok(Expr::StringLiteral(
                    token.lexeme[1..token.lexeme.len() - 1].to_string(),
                )),
                TokenType::LeftParen => {
                    let expr = self.expression()?; // must be called before consuming
                    self.consume(TokenType::RightParen, "Expected ')' after expression.")?;
                    Ok(Expr::new_grouping(expr))
                }
                TokenType::Identifier => Ok(Expr::Variable {
                    name: token.clone(),
                }),

                _ => Err(ParserError::ExpectedExpression),
            };
        }

        Err(ParserError::ExpectedExpression)
    }

    // FIXME: it should be &TokenType
    fn consume(&mut self, token_type: TokenType, message: &'static str) -> ParserResult<&Token> {
        if !self.check(&token_type) {
            return Err(ParserError::ExpectedToken { token_type });
        }

        // TODO: improve this. Maybe some kind of map?
        // We know it's the right token because of self.check
        Ok(self.advance().expect(message))
    }

    fn statement(&mut self) -> ParserResult<Stmt> {
        if self.match_tokens(&[TokenType::Print]).is_some() {
            return self.print_stmt();
        }
        if self.match_tokens(&[TokenType::LeftBrace]).is_some() {
            return self.block_stmt();
        }
        self.expression_stmt()
    }

    fn print_stmt(&mut self) -> ParserResult<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ; after value")?;
        Ok(Stmt::Print(expr))
    }

    fn block_stmt(&mut self) -> ParserResult<Stmt> {
        let mut stmts: Vec<Stmt> = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_eof() {
            stmts.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expected } after block")?;
        Ok(Stmt::Block(stmts))
    }

    fn expression_stmt(&mut self) -> ParserResult<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ; after value")?;
        Ok(Stmt::Expression(expr))
    }

    fn declaration(&mut self) -> ParserResult<Stmt> {
        if self.match_tokens(&[TokenType::Var]).is_some() {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> ParserResult<Stmt> {
        let name = self
            .consume(
                // FIXME: I missed this. I can only define the `a` variable
                TokenType::Identifier,
                "Expected variable name.",
            )?
            .clone();

        let initializer = match self.match_tokens(&[TokenType::Equal]) {
            Some(_) => Some(self.expression()?),
            None => None,
        };

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration.",
        )?;

        Ok(Stmt::Var { name, initializer })
    }
}
