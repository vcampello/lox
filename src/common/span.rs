#[derive(Debug, Clone, Default)]
pub struct Span {
    pub line: usize,
    pub col: usize,
    pub offset: (usize, usize),
}

impl Span {
    pub fn new(line: usize, col: usize, offset: (usize, usize)) -> Self {
        Self { line, col, offset }
    }

    pub fn to_location(&self) -> String {
        format!("{}, {}", self.line, self.col)
    }
}
