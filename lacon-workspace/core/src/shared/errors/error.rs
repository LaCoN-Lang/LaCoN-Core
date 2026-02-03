use super::error_type::{ErrorType, LexicalError};
use crate::frontend::lexer::token_type::TokenType;
use crate::shared::position::Position;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Expected {
	pub got: TokenType,
	pub expected: Vec<TokenType>,
}

#[derive(Debug, Clone)]
pub struct Span {
	pub start: Position,
	pub end: Position,
}

#[derive(Debug, Clone)]
pub struct Error {
	pub error_type: ErrorType,
	pub message: String,
	pub code: Option<u32>,
	pub span: Option<Span>,
	pub expected: Option<Expected>,
}

impl Error {
	pub fn new(error_type: ErrorType, message: String, code: Option<u32>, span: Option<Span>, expected: Option<Expected>) -> Self {
		Self { error_type, message, code, span, expected }
	}

	pub fn expected(got: TokenType, expected: Vec<TokenType>, start: Position, end: Position) -> Self {
		Self {
			error_type: ErrorType::Expected,
			message: format!("Unexpected token: {:?}, expected one of: {:?}", got, expected),
			code: None,
			span: Some(Span { start, end }),
			expected: Some(Expected { got, expected }),
		}
	}

	pub fn format(&self) -> String {
		let pos_str = self.span.as_ref().map_or("".to_string(), |s| format!("{}-{}", s.start, s.end));

		let expected_str = self.expected.as_ref().map_or("".to_string(), |e| {
			let expected_tokens: Vec<String> = e.expected.iter().map(|t| t.to_string()).collect();
			format!("expected {} but got {}", expected_tokens.join(" or "), e.got)
		});

		format!("[{:?}] {} {} {}", self.error_type, pos_str, self.message, expected_str)
	}
}

impl fmt::Display for LexicalError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			LexicalError::InvalidCharacter(c) => write!(f, "Invalid character '{}'", c),
			LexicalError::UnterminatedString => write!(f, "Unterminated string"),
			LexicalError::UnterminatedBlockComment => write!(f, "Unterminated block comment"),
			LexicalError::InvalidIndent => write!(f, "Invalid indentation"),
			LexicalError::InvalidEscapeSequence => write!(f, "Invalid escape sequence"),
			LexicalError::InvalidToken => write!(f, "Invalid token"),
		}
	}
}
