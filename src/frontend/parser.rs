use super::ParserError;
use crate::{
    ast::*,
    common::{Span, Spanned},
};
use std::{iter::Peekable, slice::Iter};

pub type ParserResult<T> = Result<T, ParserError>;

pub struct Parser<'a> {
    iter: Peekable<Iter<'a, Token>>,
    last_span: Span,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            iter: tokens.iter().peekable(),
            last_span: Span::default(),
        }
    }

    pub fn parse(&mut self) -> ParserResult<Vec<Stmt>> {
        let mut stmts = Vec::new();

        while matches!(self.iter.peek(), Some(token) if token.kind != TokenKind::Eof) {
            match self.declaration() {
                Ok(stmt) => stmts.push(stmt),
                Err(e) => {
                    self.synchronise();
                    return Err(e);
                }
            }
        }

        Ok(stmts)
    }

    fn synchronise(&mut self) {
        while let Some(token) = self.advance() {
            // statement boundary reached reached
            if token.kind == TokenKind::Semicolon {
                return;
            }

            if let Some(next_token) = self.iter.peek() {
                match next_token.kind {
                    TokenKind::Class
                    | TokenKind::Fun
                    | TokenKind::Var
                    | TokenKind::For
                    | TokenKind::If
                    | TokenKind::While
                    | TokenKind::Break
                    | TokenKind::Continue
                    | TokenKind::Print
                    | TokenKind::Return
                    | TokenKind::Eof => return,
                    _ => continue,
                };
            }
        }
    }

    // FIXME: this should consume
    fn advance(&mut self) -> Option<&Token> {
        let next = self.iter.next();

        if let Some(token) = next {
            self.last_span = token.span;
        }

        next
    }

    // FIXME: this should probably consume the kind
    fn check(&mut self, token_type: &TokenKind) -> bool {
        matches!(self.iter.peek(), Some(t) if t.kind == *token_type)
    }

    fn is_eof(&mut self) -> bool {
        self.check(&TokenKind::Eof)
    }

    fn match_tokens(&mut self, token_types: &[TokenKind]) -> Option<Token> {
        for token_type in token_types.iter() {
            if self.check(token_type) {
                return self.advance().cloned();
            }
        }

        None
    }

    // TODO: add production rules
    fn or(&mut self) -> ParserResult<Expr> {
        let mut expr = self.and()?;

        while let Some(token) = self.match_tokens(&[TokenKind::Or]) {
            let operator = LogicalOp::try_from(token.kind).map_err(|kind| ParserError {
                span: token.span.into(),
                kind,
            })?;
            let right = self.or()?;
            expr = Expr::logical(expr, Spanned::new(operator, token.span), right)
        }

        Ok(expr)
    }

    // TODO: add production rules
    fn and(&mut self) -> ParserResult<Expr> {
        let mut expr = self.equality()?;

        while let Some(token) = self.match_tokens(&[TokenKind::And]) {
            let operator = LogicalOp::try_from(token.kind).map_err(|kind| ParserError {
                span: token.span.into(),
                kind,
            })?;
            let right = self.equality()?;
            expr = Expr::logical(expr, Spanned::new(operator, token.span), right)
        }

        Ok(expr)
    }

    // TODO: add production rules
    fn expression(&mut self) -> ParserResult<Expr> {
        self.assignment()
    }

    // TODO: add production rules
    fn assignment(&mut self) -> ParserResult<Expr> {
        let expr = self.or()?;

        if let Some(equals) = self.match_tokens(&[TokenKind::Equal]) {
            let value = self.assignment()?;

            return match expr.kind {
                ExprKind::Variable(v) => Ok(Expr::assignment(v.var.clone(), value)),
                _ => Err(ParserError::invalid_assignment_target(
                    equals.kind,
                    equals.span,
                )),
            };
        }

        Ok(expr)
    }

    /// equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> ParserResult<Expr> {
        let mut expr = self.comparison()?;

        while let Some(token) = self.match_tokens(&[TokenKind::EqualEqual, TokenKind::BangEqual]) {
            let operator = BinaryOp::try_from(token.kind).map_err(|kind| ParserError {
                span: token.span.into(),
                kind,
            })?;
            let right = self.comparison()?;
            expr = Expr::binary(expr, Spanned::new(operator, token.span), right)
        }

        Ok(expr)
    }

    /// comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> ParserResult<Expr> {
        let mut expr = self.term()?;

        while let Some(token) = self.match_tokens(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = BinaryOp::try_from(token.kind).map_err(|kind| ParserError {
                span: token.span.into(),
                kind,
            })?;
            let right = self.term()?;
            expr = Expr::binary(expr, Spanned::new(operator, token.span), right)
        }

        Ok(expr)
    }

    /// term → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> ParserResult<Expr> {
        let mut expr = self.factor()?;

        while let Some(token) = self.match_tokens(&[TokenKind::Minus, TokenKind::Plus]) {
            let operator = BinaryOp::try_from(token.kind).map_err(|kind| ParserError {
                span: token.span.into(),
                kind,
            })?;
            let right = self.factor()?;
            expr = Expr::binary(expr, Spanned::new(operator, token.span), right)
        }

        Ok(expr)
    }

    /// factor → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> ParserResult<Expr> {
        let mut expr = self.unary()?;

        while let Some(token) = self.match_tokens(&[TokenKind::Slash, TokenKind::Star]) {
            let operator = BinaryOp::try_from(token.kind).map_err(|kind| ParserError {
                span: token.span.into(),
                kind,
            })?;
            let right = self.unary()?;
            expr = Expr::binary(expr, Spanned::new(operator, token.span), right)
        }

        Ok(expr)
    }

    /// unary → ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> ParserResult<Expr> {
        match self.match_tokens(&[TokenKind::Bang, TokenKind::Minus]) {
            Some(token) => {
                let operator = UnaryOp::try_from(token.kind).map_err(|kind| ParserError {
                    span: token.span.into(),
                    kind,
                })?;
                let right = self.unary()?;

                Ok(Expr::unary(Spanned::new(operator, token.span), right))
            }
            None => self.call(),
        }
    }

    fn return_stmt(&mut self) -> ParserResult<Stmt> {
        self.consume(TokenKind::Return, "missing return".to_string())?;

        let result = match self.check(&TokenKind::Semicolon) {
            true => None, // nil
            false => Some(self.expression()?),
        };

        self.consume(TokenKind::Semicolon, "missing ; after return".to_string())?;

        Ok(Stmt::return_stmt(result, self.last_span))
    }

    // NOTE: the book reuses this for class methods, but we'll tackle that when we get to it
    fn function_stmt(&mut self) -> ParserResult<Stmt> {
        let name = self
            .consume(TokenKind::Identifier, "expected function name".to_string())?
            .clone();
        self.consume(
            TokenKind::LeftParen,
            "expected ( after function name".to_string(),
        )?;

        let mut params: Vec<Token> = Vec::new();

        if !self.check(&TokenKind::RightParen) {
            loop {
                // TODO: remove hardcoded 255 arg limit
                if params.len() >= 255 {
                    return Err(ParserError::too_many_arguments(255, self.last_span));
                }

                let token =
                    self.consume(TokenKind::Identifier, "expected parameter name".to_string())?;
                params.push(token.to_owned());

                match self.match_tokens(&[TokenKind::Comma]) {
                    Some(_) => continue,
                    None => break,
                }
            }
        }

        self.consume(
            TokenKind::RightParen,
            "expected ) after parameters".to_string(),
        )?;

        let body = self.block_stmt()?;

        Ok(Stmt::function_stmt(name.lexeme, params, body))
    }

    fn call(&mut self) -> ParserResult<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.match_tokens(&[TokenKind::LeftParen]).is_some() {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> ParserResult<Expr> {
        let mut arguments: Vec<Expr> = Vec::new();

        if !self.check(&TokenKind::RightParen) {
            loop {
                // TODO: remove hardcoded 255 arg limit
                if arguments.len() >= 255 {
                    return Err(ParserError::too_many_arguments(255, self.last_span));
                }

                arguments.push(self.expression()?);

                if !self.check(&TokenKind::Comma) {
                    break;
                }
                self.consume(TokenKind::Comma, "missing , after arguments".to_string())?;
            }
        }

        // consume the ) at the end of the function call
        self.consume(
            TokenKind::RightParen,
            "missing ) after arguments".to_string(),
        )?;

        Ok(Expr::call(callee, arguments))
    }

    /// primary → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> ParserResult<Expr> {
        // in case there's no token
        let last_span = self.last_span;

        let token = self
            .advance()
            .ok_or(ParserError::expected_expression(last_span))?;

        match &token.kind {
            TokenKind::True => Ok(Expr::bool_literal(true, token.span)),
            TokenKind::False => Ok(Expr::bool_literal(false, token.span)),
            TokenKind::Nil => Ok(Expr::nil(token.span)),
            TokenKind::Number => token
                .lexeme
                .parse::<f64>()
                .map_err(|_| ParserError::invalid_number(token.lexeme.clone(), token.span))
                .map(|v| Expr::number_literal(v, token.span)),
            TokenKind::String => {
                // String lexeme includes quotes, strip them
                let content = token
                    .lexeme
                    .strip_prefix('"')
                    .and_then(|s| s.strip_suffix('"'))
                    .unwrap_or(&token.lexeme)
                    .to_string();
                Ok(Expr::string_literal(content, token.span))
            }

            TokenKind::LeftParen => {
                let expr = self.expression()?; // must be called before consuming
                self.consume(
                    TokenKind::RightParen,
                    "missing ) after expression".to_string(),
                )?;
                Ok(Expr::grouping(expr))
            }
            TokenKind::Identifier => Ok(Expr::variable(token.clone())),
            _ => Err(ParserError::expected_expression(self.last_span)),
        }
    }

    // ifStmt → "if" "(" expression ")" statement | ( "else" statement )? ;
    fn if_stmt(&mut self) -> ParserResult<Stmt> {
        self.consume(TokenKind::LeftParen, "missing ( after if".to_string())?;
        let condition = self.expression()?;

        self.consume(
            TokenKind::RightParen,
            "missing ) after if condition".to_string(),
        )?;
        let when_true = self.statement()?;

        let when_false = match self.match_tokens(&[TokenKind::Else]).is_some() {
            true => Some(self.statement()?),
            false => None,
        };

        Ok(Stmt::conditional_stmt(condition, when_true, when_false))
    }

    fn consume(&mut self, expected: TokenKind, message: String) -> ParserResult<&Token> {
        // in case there's no token
        let last_span = self.last_span;

        match self.advance() {
            Some(token) if token.kind == expected => Ok(token),
            Some(token) => {
                dbg!(&token);
                Err(ParserError::expected_token(
                    message, expected, token.kind, token.span,
                ))
            }
            None => Err(ParserError::unexpected_eof(message, last_span)),
        }
    }

    fn statement(&mut self) -> ParserResult<Stmt> {
        // FIXME: each one of these method calls should consume it's own starting token like
        // block_stmt
        match self.iter.peek().map(|token| &token.kind) {
            Some(TokenKind::Print) => {
                self.advance();
                self.print_stmt()
            }
            Some(TokenKind::Continue) => {
                self.advance();
                self.continue_stmt()
            }
            Some(TokenKind::Return) => {
                // self.advance();
                self.return_stmt()
            }
            Some(TokenKind::Break) => {
                self.advance();
                self.break_stmt()
            }
            Some(TokenKind::For) => {
                self.advance();
                self.for_stmt()
            }
            Some(TokenKind::While) => {
                self.advance();
                self.while_stmt()
            }
            Some(TokenKind::LeftBrace) => self.block_stmt(),
            Some(TokenKind::If) => {
                self.advance();
                self.if_stmt()
            }
            _ => self.expression_stmt(),
        }
    }

    fn print_stmt(&mut self) -> ParserResult<Stmt> {
        let expr = self.expression()?;
        self.consume(
            TokenKind::Semicolon,
            "missing ; after expression".to_string(),
        )?;

        Ok(Stmt::print_stmt(expr))
    }

    fn while_stmt(&mut self) -> ParserResult<Stmt> {
        self.consume(TokenKind::LeftParen, "missing ( after while".to_string())?;
        let condition = self.expression()?;
        self.consume(
            TokenKind::RightParen,
            "missing } after while conditon".to_string(),
        )?;
        let body = self.statement()?;

        Ok(Stmt::while_loop(condition, body))
    }

    // forStmt → "for" "(" ( varDecl | exprStmt | ";" ) expression? ";" expression? ")" statement ;
    fn for_stmt(&mut self) -> ParserResult<Stmt> {
        self.consume(TokenKind::LeftParen, "missing ( after for".to_string())?;

        let initializer = match self.match_tokens(&[TokenKind::Var, TokenKind::Semicolon]) {
            Some(token) if token.kind == TokenKind::Var => Some(self.var_declaration()?),
            Some(token) if token.kind == TokenKind::Semicolon => None,
            _ => Some(self.expression_stmt()?),
        };

        let condition = match self.check(&TokenKind::Semicolon) {
            false => Some(self.expression()?),
            true => None,
        };

        self.consume(
            TokenKind::Semicolon,
            "missing ; after for condition".to_string(),
        )?;

        let increment = match self.check(&TokenKind::RightParen) {
            false => Some(self.expression()?),
            true => None,
        };
        self.consume(
            TokenKind::RightParen,
            "missing ) after for conditon".to_string(),
        )?;
        let body = self.statement()?;

        Ok(Stmt::for_loop(initializer, condition, increment, body))
    }

    fn block_stmt(&mut self) -> ParserResult<Stmt> {
        self.consume(TokenKind::LeftBrace, "missing { before block".to_string())?;

        let mut stmts: Vec<Stmt> = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_eof() {
            stmts.push(self.declaration()?);
        }

        self.consume(TokenKind::RightBrace, "missing } after block".to_string())?;

        Ok(Stmt::block_stmt(stmts, self.last_span))
    }

    fn expression_stmt(&mut self) -> ParserResult<Stmt> {
        let expr = self.expression()?;
        self.consume(
            TokenKind::Semicolon,
            "missing ; after expression".to_string(),
        )?;

        Ok(Stmt::expr_stmt(expr))
    }

    fn continue_stmt(&mut self) -> ParserResult<Stmt> {
        self.consume(TokenKind::Semicolon, "missing ; after continue".to_string())?;

        Ok(Stmt::continue_stmt(self.last_span))
    }

    fn break_stmt(&mut self) -> ParserResult<Stmt> {
        self.consume(TokenKind::Semicolon, "missing ; after break".to_string())?;

        Ok(Stmt::break_stmt(self.last_span))
    }

    fn declaration(&mut self) -> ParserResult<Stmt> {
        match self.iter.peek().map(|token| &token.kind) {
            Some(TokenKind::Fun) => {
                self.advance();
                self.function_stmt()
            }
            Some(TokenKind::Var) => {
                self.advance();
                self.var_declaration()
            }
            _ => self.statement(),
        }
    }

    fn var_declaration(&mut self) -> ParserResult<Stmt> {
        let name = self
            .consume(TokenKind::Identifier, "missing variable name".to_string())?
            .clone();

        let initializer = match self.match_tokens(&[TokenKind::Equal]) {
            Some(_) => Some(self.expression()?),
            None => None,
        };

        self.consume(
            TokenKind::Semicolon,
            "missing ; after variable declaration".to_string(),
        )?;

        Ok(Stmt::variable_stmt(name, initializer))
    }
}
