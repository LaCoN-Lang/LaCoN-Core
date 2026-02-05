use super::TokenKind;
use crate::shared::position::Position;
use bitflags::bitflags;

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

	pub token_kind: TokenKind,
	pub length: u32,
	pub flags: TokenFlags,
}
