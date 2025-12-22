use std::fmt;

pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match *self {
            // Single-character tokens.
            TokenType::LeftParen => "LeftParen",
            TokenType::RightParen => "RightParen",
            TokenType::LeftBrace => "LeftBrace",
            TokenType::RightBrace => "RightBrace",
            TokenType::Comma => "Comma",
            TokenType::Dot => "Dot",
            TokenType::Minus => "Minus",
            TokenType::Plus => "Plus",
            TokenType::Semicolon => "Semicolon",
            TokenType::Slash => "Slash",
            TokenType::Star => "Star",

            // One or two character tokens.
            TokenType::Bang => "Bang",
            TokenType::BangEqual => "BangEqual",
            TokenType::Equal => "Equal",
            TokenType::EqualEqual => "EqualEqual",
            TokenType::Greater => "Greater",
            TokenType::GreaterEqual => "GreaterEqual",
            TokenType::Less => "Less",
            TokenType::LessEqual => "LessEqual",

            // Literals.
            TokenType::Identifier => "Identifier",
            TokenType::String => "String",
            TokenType::Number => "Number",

            // Keywords.
            TokenType::And => "And",
            TokenType::Class => "Class",
            TokenType::Else => "Else",
            TokenType::False => "False",
            TokenType::Fun => "Fun",
            TokenType::For => "For",
            TokenType::If => "If",
            TokenType::Nil => "Nil",
            TokenType::Or => "Or",
            TokenType::Print => "Print",
            TokenType::Return => "Return",
            TokenType::Super => "Super",
            TokenType::This => "This",
            TokenType::True => "True",
            TokenType::Var => "Var",
            TokenType::While => "While",

            TokenType::Eof => "Eof",
        };

        write!(f, "{value}")
    }
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    // REFACTOR: this needs to be generic. Maybe the TokenType needs to have the payload?
    // The book uses `Object literal` because it represents the value of the token - e.g. a number
    // or string for a variable.
    literal: String,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}
