use super::error_type::ErrorType;
use crate::shared::position::Position;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Error {
	pub error_type: ErrorType,
	pub start: Option<Position>,
	pub end: Option<Position>,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}]", self.error_type)?;

		if let Some(start_pos) = &self.start {
			write!(f, " at {}", start_pos)?;
		}

		if let Some(end_pos) = &self.end {
			write!(f, " to {}", end_pos)?;
		}

		Ok(())
	}
}

impl Error {
	pub fn new(error_type: ErrorType, start: Option<Position>, end: Option<Position>) -> Self {
		Self { error_type, start, end }
	}
}
