use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
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
        // local import to make this easier to maintain
        use TokenKind::*;

        match keyword {
            // Keywords.
            "and" => And,
            "class" => Class,
            "else" => Else,
            "false" => False,
            "fun" => Fun,
            "for" => For,
            "if" => If,
            "nil" => Nil,
            "or" => Or,
            "print" => Print,
            "return" => Return,
            "super" => Super,
            "this" => This,
            "true" => True,
            "var" => Var,
            "while" => While,
            "continue" => Continue,
            "break" => Break,

            // Not a keyword
            _ => Identifier,
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // local import to make this easier to maintain
        use TokenKind::*;

        match self {
            // Single-character tokens.
            LeftParen => write!(f, "LeftParen"),
            RightParen => write!(f, "RightParen"),
            LeftBrace => write!(f, "LeftBrace"),
            RightBrace => write!(f, "RightBrace"),
            Comma => write!(f, "Comma"),
            Dot => write!(f, "Dot"),
            Minus => write!(f, "Minus"),
            Plus => write!(f, "Plus"),
            Semicolon => write!(f, "Semicolon"),
            Slash => write!(f, "Slash"),
            Star => write!(f, "Star"),
            QuestionMark => write!(f, "QuestionMark"),
            Colon => write!(f, "Colon"),

            // One or two character tokens.
            Bang => write!(f, "Bang"),
            BangEqual => write!(f, "BangEqual"),
            Equal => write!(f, "Equal"),
            EqualEqual => write!(f, "EqualEqual"),
            Greater => write!(f, "Greater"),
            GreaterEqual => write!(f, "GreaterEqual"),
            Less => write!(f, "Less"),
            LessEqual => write!(f, "LessEqual"),

            // Literals.
            Identifier => write!(f, "Identifier"),
            String => write!(f, "String"),
            Number => write!(f, "Number"),

            // Keywords.
            And => write!(f, "And"),
            Class => write!(f, "Class"),
            Else => write!(f, "Else"),
            False => write!(f, "False"),
            Fun => write!(f, "Fun"),
            For => write!(f, "For"),
            If => write!(f, "If"),
            Nil => write!(f, "Nil"),
            Or => write!(f, "Or"),
            Print => write!(f, "Print"),
            Return => write!(f, "Return"),
            Super => write!(f, "Super"),
            This => write!(f, "This"),
            True => write!(f, "True"),
            Var => write!(f, "Var"),
            While => write!(f, "While"),
            Continue => write!(f, "Continue"),
            Break => write!(f, "Break"),

            // Misc.
            Eof => write!(f, "Eof"),
        }
    }
}
