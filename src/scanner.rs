use std::{iter::Peekable, str::Chars};

use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    tokens: Vec<Token>,
    // TODO: convert to vec as nth is expensive
    source: &'a str,

    /// A shared iterator over the source
    ///
    /// This iterator will always return None after it's exhausted. This is not always the case
    /// for other types:
    /// - https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next
    /// - https://doc.rust-lang.org/std/iter/trait.FusedIterator.html
    chars: Peekable<Chars<'a>>,

    // source.len() returns usize and these properties are derived from it
    /// Lexeme start
    start: usize, // start

    /// Current character position in the source code
    current: usize,

    /// Current line in the source code
    line: usize,
}

impl<'a> Scanner<'a> {
    // TODO: accept a stream
    pub fn new(source: &'a str) -> Self {
        Self {
            tokens: Vec::new(),
            source,
            chars: source.chars().peekable(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        // scan each character
        while let Some(char) = self.advance() {
            // Look at the current and next character
            match (char, self.chars.peek()) {
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
                ('!', Some('=')) => self.add_token_and_skip(TokenType::BangEqual, 1),
                ('!', _) => self.add_token(TokenType::Bang),

                // equality
                ('=', Some('=')) => self.add_token_and_skip(TokenType::EqualEqual, 1),
                ('=', _) => self.add_token(TokenType::Equal),

                // greater than
                ('>', Some('=')) => self.add_token_and_skip(TokenType::GreaterEqual, 1),
                ('>', _) => self.add_token(TokenType::Greater),

                // greater than
                ('<', Some('=')) => self.add_token_and_skip(TokenType::LessEqual, 1),
                ('<', _) => self.add_token(TokenType::Less),

                // slash or comment
                ('/', Some('/')) => self.handle_comment(),
                ('/', _) => self.add_token(TokenType::Slash),

                // misc
                ('?', _) => self.add_token(TokenType::QuestionMark),
                (':', _) => self.add_token(TokenType::Colon),

                // whitespace
                (' ', _) => (),
                ('\t', _) => (),
                ('\r', _) => (),
                ('\n', _) => {
                    self.line += 1;
                }

                // literals
                ('"', _) => self.handle_string(),
                (char, _) if char.is_ascii_digit() => self.handle_number(),
                (char, _) if Scanner::is_identifier(&char) => self.handle_identifier_and_keywords(),

                // REFACTOR: there's some shared error handling between the scanner and the runtime
                (token, _) => eprintln!(" {}| Unknown token: {token}", self.line),
            };

            // set lexeme start
            self.start = self.current;
        }

        // TODO: completely remove Eof once the book explains why it needs it
        // Terminate the program
        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), self.line));

        &self.tokens
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = &self.source[self.start..self.current];
        let token = Token::new(token_type, lexeme.to_string(), self.line);
        self.tokens.push(token);
    }

    fn add_token_and_skip(&mut self, token_type: TokenType, skip_chars: usize) {
        self.add_token(token_type);

        // skip n chars
        for _ in 0..skip_chars {
            self.advance();
        }
    }

    /// Consume current character. Increases character index if next() is Some(_)
    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next();

        // prevent out of bound lookups when indexing the source array
        if c.is_some() {
            self.current += 1;
        }

        c
    }

    fn consume_digits(&mut self) {
        while matches!(self.chars.peek(), Some(c) if c.is_ascii_digit()) {
            self.advance();
        }
    }

    fn handle_comment(&mut self) {
        // consume the next character
        while let Some(c) = self.advance() {
            if c == '\n' {
                break;
            }
        }
    }

    fn handle_string(&mut self) {
        while let Some(c) = self.chars.peek() {
            match c {
                // Multi-line comment
                '\n' => self.line += 1,
                '"' => break,
                _ => (),
            };
            _ = self.advance();
        }

        if self.chars.peek().is_none() {
            // REFACTOR: search for eprintln in this file and consolidate them
            eprintln!(" {}| Unterminated string.", self.line);
            return;
        }

        // consume closing "
        _ = self.advance();

        let literal = &self.source[self.start + 1..self.current - 1];

        self.add_token(TokenType::String(literal.to_string()));
    }

    fn handle_number(&mut self) {
        // consume whole number
        self.consume_digits();

        // create a second iterator to look farther
        let mut chars2 = self.chars.clone();

        let peek_1st = self.chars.peek();
        let peek_2nd = {
            chars2.next();
            chars2.peek()
        };

        // check if the current and next characters are the fractional part of a number -e.g. `.9`
        match (peek_1st, peek_2nd) {
            (Some(c), Some(next)) if *c == '.' && next.is_ascii_digit() => {
                // consume '.'
                self.advance();

                // consume fractional
                self.consume_digits();
            }
            _ => (),
        };

        let literal = &self.source[self.start..self.current];
        let Ok(number) = literal.parse::<f64>() else {
            eprintln!(" {}| failed to parse({literal})", self.line,);
            return;
        };

        self.add_token(TokenType::Number(number));
    }

    fn handle_identifier_and_keywords(&mut self) {
        // extract the entire identifier before categorising it. See maximal munch
        while matches!(self.chars.peek(), Some(c) if Scanner::is_identifier(c)) {
            self.advance();
        }

        let identifier = &self.source[self.start..self.current];

        // Check if it's a reserved keyword
        let token_type = match TokenType::matching_identifier(identifier) {
            Some(t) => t,
            _ => TokenType::Identifier(identifier.to_string()),
        };

        self.add_token(token_type);
    }

    fn is_identifier(c: &char) -> bool {
        c.is_ascii_alphanumeric() || *c == '_'
    }
}
