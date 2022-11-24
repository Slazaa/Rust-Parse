use std::fmt;

#[derive(Clone, Debug)]
pub struct Position {
	pub idx: usize,
	pub line: usize,
	pub col: usize
}

impl fmt::Display for Position {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}:{}", self.line, self.col)
	}
}

impl Default for Position {
	fn default() -> Self {
		Self { idx: 0, line: 1, col: 1 }
	}
}

#[derive(Clone, Debug, Default)]
pub struct Location {
	pub filename: Option<String>,
	pub start: Position,
	pub end: Position
}

#[derive(Clone, Debug)]
pub struct Token {
	pub name: String,
	pub symbol: String,
	pub loc: Location
}