use crate::ast::{Token, TokenKind};
use crate::{common::Span, frontend::ScannerError};
use std::{iter::Peekable, str::Chars};

pub type ScannerResult<T> = Result<T, ScannerError>;

pub struct Scanner<'a> {
    tokens: Vec<Token>,
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

    /// Current line in the source code (0 indexed)
    line: usize,

    /// Current column in the source code (0 indexed)
    col: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            tokens: Vec::new(),
            source,
            chars: source.chars().peekable(),
            start: 0,
            current: 0,
            line: 0,
            col: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> ScannerResult<&Vec<Token>> {
        // scan each character
        while let Some(char) = self.advance() {
            // Look at the current and next character
            match (char, self.chars.peek()) {
                ('(', _) => self.add_token(TokenKind::LeftParen),
                (')', _) => self.add_token(TokenKind::RightParen),
                ('{', _) => self.add_token(TokenKind::LeftBrace),
                ('}', _) => self.add_token(TokenKind::RightBrace),
                (',', _) => self.add_token(TokenKind::Comma),
                ('.', _) => self.add_token(TokenKind::Dot),
                ('-', _) => self.add_token(TokenKind::Minus),
                ('+', _) => self.add_token(TokenKind::Plus),
                (';', _) => self.add_token(TokenKind::Semicolon),
                ('*', _) => self.add_token(TokenKind::Star),

                // negation
                ('!', Some('=')) => self.add_token_and_skip(TokenKind::BangEqual, 1),
                ('!', _) => self.add_token(TokenKind::Bang),

                // equality
                ('=', Some('=')) => self.add_token_and_skip(TokenKind::EqualEqual, 1),
                ('=', _) => self.add_token(TokenKind::Equal),

                // greater than
                ('>', Some('=')) => self.add_token_and_skip(TokenKind::GreaterEqual, 1),
                ('>', _) => self.add_token(TokenKind::Greater),

                // greater than
                ('<', Some('=')) => self.add_token_and_skip(TokenKind::LessEqual, 1),
                ('<', _) => self.add_token(TokenKind::Less),

                // slash or comment
                ('/', Some('/')) => self.handle_comment(),
                ('/', Some('*')) => self.handle_block_comment(), // add an error production for */
                ('/', _) => self.add_token(TokenKind::Slash),

                // misc
                ('?', _) => self.add_token(TokenKind::QuestionMark),
                (':', _) => self.add_token(TokenKind::Colon),

                // whitespace
                (' ', _) => (),
                ('\t', _) => (),
                ('\r', _) => (),
                ('\n', _) => self.increase_line(),

                // literals
                ('"', _) => self.handle_string()?,
                (ch, _) if ch.is_ascii_digit() => self.handle_number(),
                (ch, _) if Scanner::is_identifier(&ch) => self.handle_identifier_and_keywords(),

                (ch, _) => {
                    return Err(ScannerError::unexpected_character(ch, self.to_span_all()));
                }
            };

            // set lexeme start
            self.start = self.current;
        }

        self.add_token(TokenKind::Eof);

        Ok(&self.tokens)
    }

    fn to_span_all(&self) -> Span {
        Span {
            line: self.line,
            col: self.col,
            offset: self.start,
            length: self.current - self.start,
        }
    }

    fn to_span_single(&self) -> Span {
        Span {
            line: self.line,
            col: self.col,
            offset: self.current.saturating_sub(1),
            length: 1,
        }
    }

    fn increase_line(&mut self) {
        self.line += 1;
        self.col = 1;
    }

    fn add_token(&mut self, token_type: TokenKind) {
        let lexeme = &self.source[self.start..self.current];
        let token = Token::new(token_type, lexeme.to_string(), self.to_span_all());
        self.tokens.push(token);
    }

    fn add_sring_token(&mut self, s: String) {
        let token = Token::new(TokenKind::String, s, self.to_span_all());
        self.tokens.push(token);
    }

    fn add_token_and_skip(&mut self, token_type: TokenKind, skip_chars: usize) {
        self.add_token(token_type);

        // skip n chars
        for _ in 0..skip_chars {
            if self.advance().is_none() {
                break;
            }
        }
    }

    /// Consume current character. Increases character index if next() is Some(_)
    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next();

        // prevent out of bound lookups when indexing the source array
        if c.is_some() {
            self.current += 1;
            self.col += 1;
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

    fn handle_block_comment(&mut self) {
        // consume the next character
        while let Some(c) = self.advance() {
            if c == '*' && matches!(self.chars.peek(), Some('/')) {
                // consume / too
                self.advance();
                break;
            }
        }
    }

    fn handle_string(&mut self) -> ScannerResult<()> {
        // FIXME: for now this will live here, but ideally all the parsing (including numbers)
        // should live in the parser - ie. the parser should do this from the span
        // store final string value
        let mut buf: Vec<char> = Vec::new();
        while let Some(cur) = self.advance() {
            match cur {
                // multi-line string handling
                '\n' => {
                    buf.push(cur);
                    self.increase_line();
                }
                // handle escape sequences
                '\\' if let Some(next) = self.chars.peek() => match next {
                    // escaped slashes and quotes inside of strings
                    '"' => {
                        self.advance();
                        buf.push('"');
                    }
                    '\\' => {
                        self.advance();
                        buf.push('\\');
                    }
                    // tab
                    't' => {
                        self.advance();
                        buf.push('\t');
                    }
                    // newline
                    'n' => {
                        self.advance();
                        buf.push('\n');
                    }
                    // carriage return
                    'r' => {
                        self.advance();
                        buf.push('\r');
                    }
                    // anything else
                    _ => buf.push(cur),
                },
                // terminate string
                '"' => {
                    self.add_sring_token(buf.iter().collect::<String>());
                    return Ok(());
                }
                // append to buffer
                _ => buf.push(cur),
            };
        }

        Err(ScannerError::unterminated_string(self.to_span_single()))
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

        self.add_token(TokenKind::Number);
    }

    fn handle_identifier_and_keywords(&mut self) {
        // extract the entire identifier before categorising it. See maximal munch
        while matches!(self.chars.peek(), Some(c) if Scanner::is_identifier(c)) {
            self.advance();
        }

        let identifier = &self.source[self.start..self.current];

        // Convert to keyword or identifier
        self.add_token(TokenKind::to_identifier(identifier));
    }

    fn is_identifier(c: &char) -> bool {
        c.is_ascii_alphanumeric() || *c == '_'
    }
}
