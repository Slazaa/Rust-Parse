use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Position {
	idx: usize,
	line: usize,
	col: usize
}

impl Position {
	pub fn new(idx: usize, line: usize, col: usize) -> Self {
		Self {
			idx,
			line,
			col
		}
	}

	pub fn idx(&self) -> usize {
		self.idx
	}

	pub fn line(&self) -> usize {
		self.line
	}

	pub fn col(&self) -> usize {
		self.col
	}

	pub fn idx_mut(&mut self) -> &mut usize {
		&mut self.idx
	}

	pub fn line_mut(&mut self) -> &mut usize {
		&mut self.line
	}

	pub fn col_mut(&mut self) -> &mut usize {
		&mut self.col
	}
}

impl fmt::Display for Position {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}:{}", self.line, self.col)
	}
}

#[derive(Clone, Debug)]
pub struct Token {
	name: String,
	symbol: String,
	start_pos: Position,
	end_pos: Position
}

impl Token {
	pub fn new(name: &str, symbol: &str, start_pos: &Position, end_pos: &Position) -> Self {
		Self {
			name: name.to_owned(),
			symbol: symbol.to_owned(),
			start_pos: *start_pos,
			end_pos: *end_pos
		}
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn symbol(&self) -> &String {
		&self.symbol
	}

	pub fn start_pos(&self) -> &Position {
		&self.start_pos
	}

	pub fn end_pos(&self) -> &Position {
		&self.end_pos
	}
}