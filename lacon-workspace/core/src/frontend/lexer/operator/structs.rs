use super::super::TokenKind;

pub struct OpMatch {
	pub token_kind: TokenKind,
	pub consume_count: usize,
}
