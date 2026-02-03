use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
	pub line: usize,
	pub column: usize,
	pub offset: usize,
}

impl Position {
	pub fn start() -> Self {
		Self { line: 1, column: 1, offset: 0 }
	}

	pub fn new(line: usize, column: usize, offset: usize) -> Self {
		Self { line, column, offset }
	}

	pub fn advance(&mut self, ch: char) {
		self.offset += ch.len_utf8();

		if ch == '\n' {
			self.line += 1;
			self.column = 1;
		} else {
			self.column += 1;
		}
	}

	pub fn shifted(&self, ch: char) -> Self {
		let mut new_pos = *self;
		new_pos.advance(ch);
		new_pos
	}
}

impl fmt::Display for Position {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}:{}", self.line, self.column)
	}
}

impl std::ops::Add<usize> for Position {
	type Output = Position;

	fn add(self, rhs: usize) -> Self::Output {
		Position {
			line: self.line,
			column: self.column + rhs,
			offset: self.offset + rhs,
		}
	}
}
