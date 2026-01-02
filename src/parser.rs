use crate::token::Token;

pub struct Parser<'a> {
    current: usize,
    tokens: &'a [Token],
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current: 0 }
    }
    fn advance(&mut self) {
        //
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
