use crate::token::{Token, TokenType};

pub struct Scanner {
    tokens: Vec<Token>,
    // TODO: convert to vec as nth is expensive
    source: String,
    // source.len() returns usize and these properties are derived from it
    /// Lexeme start
    start: usize, // start
    /// Current character position in the source code
    current: usize,
    /// Current line in the source code
    line: usize,
}

impl Scanner {
    // TODO: accept a stream
    pub fn new(source: &str) -> Self {
        Self {
            tokens: Vec::new(),
            source: source.to_string(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = &self.source[self.start..self.current];
        let token = Token::new(token_type, lexeme.into(), String::new(), self.line);
        self.tokens.push(token);
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        // TODO: would it be better to use an iterator?
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            String::new(),
            self.line,
        ));

        &self.tokens
    }

    pub fn scan_token(&mut self) {
        let Some(c) = self.next_char() else { return };

        // REVIEW: use add_token for now, but we'll probably have to built the tokens here to
        // attach the literal value.

        // Look at the current and next character
        match (c, self.peek_next()) {
            ('(', _) => self.add_token(TokenType::LeftParen),
            (')', _) => self.add_token(TokenType::RightParen),
            ('{', _) => self.add_token(TokenType::LeftBrace),
            ('}', _) => self.add_token(TokenType::RightBrace),
            (',', _) => self.add_token(TokenType::Comma),
            ('.', _) => self.add_token(TokenType::Dot),
            ('-', _) => self.add_token(TokenType::Minus),
            ('+', _) => self.add_token(TokenType::Plus),
            (';', _) => self.add_token(TokenType::Semicolon),
            ('*', _) => self.add_token(TokenType::Star),

            // negation
            ('!', Some('=')) => {
                // consume the next character
                self.next_char();
                self.add_token(TokenType::BangEqual);
            }
            ('!', _) => self.add_token(TokenType::Bang),

            // equality
            ('=', Some('=')) => {
                // consume the next character
                self.next_char();
                self.add_token(TokenType::EqualEqual);
            }
            ('=', _) => self.add_token(TokenType::Equal),

            // greater than
            ('>', Some('=')) => {
                // consume the next character
                self.next_char();
                self.add_token(TokenType::GreaterEqual);
            }
            ('>', _) => self.add_token(TokenType::Greater),
            // greater than
            ('<', Some('=')) => {
                // consume the next character
                self.next_char();
                self.add_token(TokenType::LessEqual);
            }
            ('<', _) => self.add_token(TokenType::Less),

            // slash or comment
            ('/', Some('/')) => {
                // consume the next character
                while let Some(c) = self.next_char()
                    && !self.is_at_end()
                {
                    if c == '\n' {
                        break;
                    }
                }
            }
            ('/', _) => self.add_token(TokenType::Slash),

            // REFACTOR: there's some shared error handling between the scanner and the runtime
            (token, _) => eprintln!("[line {}] Unknown token: {token}", self.line),
        };
    }

    // called `advance` in the book
    pub fn next_char(&mut self) -> Option<char> {
        // REVIEW:
        // this is required because we only want to look at one character at a time. Perhaps
        // there's a better way to do it.
        let c = self.source.chars().nth(self.current);

        if c.is_some() {
            self.current += 1;
        }

        c
    }

    // called `peek` in the book
    pub fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }
}
