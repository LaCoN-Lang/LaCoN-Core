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
		let start_str = self.start.as_ref().map_or("".to_string(), |p| format!(" at {}", p));
		let end_str = self.end.as_ref().map_or("".to_string(), |p| format!(" to {}", p));
		write!(f, "[{}]{}{}", self.error_type, start_str, end_str)
	}
}

impl Error {
	pub fn new(error_type: ErrorType, start: Option<Position>, end: Option<Position>) -> Self {
		Self { error_type, start, end }
	}
}
