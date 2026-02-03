use crate::frontend::lexer::position::Position;
use crate::frontend::lexer::token_type::TokenType;
use bitflags::bitflags;
use std::fmt;

bitflags! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub struct TokenFlags: u8 {
		const AT_LINE_START = 0b0000_0001;
		const HAS_PRECEDING_WHITESPACE = 0b0000_0010;
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
	pub lexeme: String,
	pub literal: Option<String>,
	pub position: Position,

	pub token_type: TokenType,
	pub length: u32,
	pub flags: TokenFlags,
}

impl Token {
	pub fn new(token_type: TokenType, is_at_line_start: bool, has_whitespace: bool, lexeme: String, literal: Option<String>, position: Position, length: usize) -> Self {
		let mut flags = TokenFlags::empty();
		if is_at_line_start {
			flags.insert(TokenFlags::AT_LINE_START);
		}
		if has_whitespace {
			flags.insert(TokenFlags::HAS_PRECEDING_WHITESPACE);
		}

		Self {
			token_type,
			lexeme,
			literal,
			position,
			length: length as u32,
			flags,
		}
	}

	pub fn bare(token_type: TokenType, position: Position) -> Self {
		Self {
			token_type,
			lexeme: String::new(),
			literal: None,
			position,
			length: 0,
			flags: TokenFlags::empty(),
		}
	}

	pub fn error(message: String, position: Position) -> Self {
		Self {
			token_type: TokenType::Error,
			lexeme: message,
			literal: None,
			position,
			length: 0,
			flags: TokenFlags::empty(),
		}
	}
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let literal_str = match &self.literal {
			Some(l) => format!(" (value: {})", l),
			None => String::new(),
		};

		let mut markers = String::new();
		if self.flags.contains(TokenFlags::AT_LINE_START) {
			markers.push_str(" [SOL]"); // Start of Line
		}
		if self.flags.contains(TokenFlags::HAS_PRECEDING_WHITESPACE) {
			markers.push_str(" [WS]"); // Whitespace
		}

		write!(f, "[{:?}{}] '{}'{} at {}", self.token_type, markers, self.lexeme, literal_str, self.position)
	}
}
