use miette::SourceSpan;

#[derive(Debug, Clone, Default, Copy, PartialEq, PartialOrd)]
pub struct Span {
    /// source code end line (0 indexed)
    pub line: usize,
    /// source code end column (0 indexed)
    pub col: usize,
    /// source code start offset
    pub offset: usize,
    /// source code length from start offset
    pub length: usize,
}

impl Span {
    pub fn to_location(self) -> String {
        format!("{}, {}", self.line, self.col)
    }

    pub fn merge(&self, other: &Span) -> Self {
        let col = self.col.min(other.col);
        let line = self.line.min(other.line);
        let offset = self.offset.min(other.offset);

        let a_end = self.offset + self.length;
        let b_end = other.offset + other.length;
        let end = a_end.max(b_end);

        let length = end - offset;

        Self {
            col,
            line,
            offset,
            length,
        }
    }
}

// convenience trait to convert to miette's SourceSpan with span.into()
impl From<Span> for SourceSpan {
    fn from(value: Span) -> Self {
        Self::from((value.offset, value.length))
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(value: T, span: Span) -> Self {
        Self { span, value }
    }
}

impl<T> std::ops::Deref for Spanned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
