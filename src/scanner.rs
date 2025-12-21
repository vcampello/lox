use crate::token::TokenType;

pub struct Scanner {}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}

impl Scanner {
    pub fn new() -> Self {
        Self {}
    }

    pub fn scan_tokens(self) -> Vec<TokenType> {
        vec![]
    }
}
