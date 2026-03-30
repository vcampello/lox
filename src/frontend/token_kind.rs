use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
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
    QuestionMark,
    Colon,

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
    Continue,
    Break,

    // Misc
    Eof,
}

impl TokenKind {
    pub fn to_identifier(keyword: &str) -> TokenKind {
        match keyword {
            // Keywords.
            "and" => TokenKind::And,
            "class" => TokenKind::Class,
            "else" => TokenKind::Else,
            "false" => TokenKind::False,
            "fun" => TokenKind::Fun,
            "for" => TokenKind::For,
            "if" => TokenKind::If,
            "nil" => TokenKind::Nil,
            "or" => TokenKind::Or,
            "print" => TokenKind::Print,
            "return" => TokenKind::Return,
            "super" => TokenKind::Super,
            "this" => TokenKind::This,
            "true" => TokenKind::True,
            "var" => TokenKind::Var,
            "while" => TokenKind::While,
            "continue" => TokenKind::Continue,
            "break" => TokenKind::Break,

            // Not a keyword
            _ => TokenKind::Identifier,
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: write a macro for this
        match self {
            // Single-character tokens.
            TokenKind::LeftParen => write!(f, "LeftParen"),
            TokenKind::RightParen => write!(f, "RightParen"),
            TokenKind::LeftBrace => write!(f, "LeftBrace"),
            TokenKind::RightBrace => write!(f, "RightBrace"),
            TokenKind::Comma => write!(f, "Comma"),
            TokenKind::Dot => write!(f, "Dot"),
            TokenKind::Minus => write!(f, "Minus"),
            TokenKind::Plus => write!(f, "Plus"),
            TokenKind::Semicolon => write!(f, "Semicolon"),
            TokenKind::Slash => write!(f, "Slash"),
            TokenKind::Star => write!(f, "Star"),
            TokenKind::QuestionMark => write!(f, "QuestionMark"),
            TokenKind::Colon => write!(f, "Colon"),

            // One or two character tokens.
            TokenKind::Bang => write!(f, "Bang"),
            TokenKind::BangEqual => write!(f, "BangEqual"),
            TokenKind::Equal => write!(f, "Equal"),
            TokenKind::EqualEqual => write!(f, "EqualEqual"),
            TokenKind::Greater => write!(f, "Greater"),
            TokenKind::GreaterEqual => write!(f, "GreaterEqual"),
            TokenKind::Less => write!(f, "Less"),
            TokenKind::LessEqual => write!(f, "LessEqual"),

            // Literals.
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::String => write!(f, "String"),
            TokenKind::Number => write!(f, "Number"),

            // Keywords.
            TokenKind::And => write!(f, "And"),
            TokenKind::Class => write!(f, "Class"),
            TokenKind::Else => write!(f, "Else"),
            TokenKind::False => write!(f, "False"),
            TokenKind::Fun => write!(f, "Fun"),
            TokenKind::For => write!(f, "For"),
            TokenKind::If => write!(f, "If"),
            TokenKind::Nil => write!(f, "Nil"),
            TokenKind::Or => write!(f, "Or"),
            TokenKind::Print => write!(f, "Print"),
            TokenKind::Return => write!(f, "Return"),
            TokenKind::Super => write!(f, "Super"),
            TokenKind::This => write!(f, "This"),
            TokenKind::True => write!(f, "True"),
            TokenKind::Var => write!(f, "Var"),
            TokenKind::While => write!(f, "While"),
            TokenKind::Continue => write!(f, "Continue"),
            TokenKind::Break => write!(f, "Break"),

            // Misc.
            TokenKind::Eof => write!(f, "Eof"),
        }
    }
}
