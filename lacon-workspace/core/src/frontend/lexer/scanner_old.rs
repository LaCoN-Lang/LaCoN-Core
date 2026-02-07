use super::{SyntaxKind, Token, TokenKind, get_keyword_token, match_operator};
use crate::shared::{Error, ErrorFlag, ErrorKind, ErrorStorage, LexicalError, Position, UnitContext, UnitKind};
use unicode_ident::{is_xid_continue, is_xid_start};

use lasso::{Rodeo, Spur};
use std::str::Chars;

const EOF_CHAR: char = '\0';

#[derive(Debug)]
pub struct Scanner<'src> {
	source: &'src str,
	context: &'src UnitContext<'src>,

	errors_storage: &'src mut ErrorStorage,
	tokens: Vec<Token<'src>>,
	start: usize,
	current: usize,
	chars: Chars<'src>,
	position: Position,
	start_position: Position,
	indent_stack: Vec<usize>,
	context_stack: Vec<TokenKind>,
	string_stack: Vec<(char, bool)>,
	is_at_line_start: bool,
	had_whitespace: bool,
	#[cfg(debug_assertions)]
	prev: char,
}

impl<'src> Scanner<'src> {
	pub fn reset(&mut self, new_source: &'src str) {
		self.source = new_source;
		self.chars = new_source.chars();

		self.start = 0;
		self.current = 0;
		self.tokens.clear();

		self.position = Position::start();
		self.start_position = Position::start();

		self.indent_stack.clear();
		self.indent_stack.push(0);
		self.context_stack.clear();
		self.string_stack.clear();

		self.is_at_line_start = true;
		self.had_whitespace = false;

		#[cfg(debug_assertions)]
		{
			self.prev = EOF_CHAR;
		}
	}
}

impl<'src> Scanner<'src> {
	pub fn new(source: &'src str, ctx: &'src UnitContext, errors_storage: &'src mut ErrorStorage) -> Self {
		let start_pos = Position::start();
		Self {
			source,
			context: ctx,
			errors_storage,
			tokens: Vec::new(),
			start: 0,
			current: 0,
			chars: source.chars(),
			position: start_pos,
			start_position: start_pos,
			indent_stack: vec![0],
			context_stack: Vec::new(),
			string_stack: Vec::new(),
			is_at_line_start: true,
			had_whitespace: false,
			#[cfg(debug_assertions)]
			prev: EOF_CHAR,
		}
	}

	pub fn scan_tokens(&mut self) -> &Vec<Token<'_>> {
		self.tokens.push(Token::bare(TokenKind::SOF, self.position));

		while !self.is_at_end() {
			self.start = self.current;
			self.start_position = self.position;

			if self.is_at_line_start {
				self.handle_indentation();
			}

			if !self.is_at_end() {
				self.scan_token();
			}
		}

		while self.indent_stack.len() > 1 {
			self.indent_stack.pop();
			let level = (self.indent_stack.len() - 1) as u8;
			self.add_token_raw(TokenKind::Dedent(level));
		}

		self.tokens.push(Token::bare(TokenKind::EOF, self.position));

		&self.tokens
	}

	fn scan_token(&mut self) {
		let c = self.advance();
		let first = self.first();
		let second = self.second();

		match c {
			// ' ' | '\t' | '\r' => {
			// 	self.had_whitespace = true;
			// 	self.start = self.current;
			// 	self.start_position = self.position;
			// }
			// '\n' => {
			// 	self.add_token_raw(TokenKind::Newline);
			// 	self.is_at_line_start = true;
			// 	self.had_whitespace = false;
			// 	self.start = self.current;
			// 	self.start_position = self.position;
			// }

			// '"' | '\'' | '`' => {
			// 	self.scan_string(c);
			// }

			// ';' => self.add_token(TokenKind::Syntax(SyntaxKind::Semicolon)),
			// ',' => self.add_token(TokenKind::Syntax(SyntaxKind::Comma)),
			// '$' => self.add_token(TokenKind::Syntax(SyntaxKind::Dollar)),
			// '@' => self.add_token(TokenKind::Syntax(SyntaxKind::At)),
			// '#' => self.add_token(TokenKind::Syntax(SyntaxKind::Hash)),
			// '\\' => self.add_token(TokenKind::Syntax(SyntaxKind::Backslash)),

			// '(' => {
			// 	self.context_stack.push(TokenKind::Syntax(SyntaxKind::LeftParenthesis));
			// 	self.add_token(TokenKind::Syntax(SyntaxKind::LeftParenthesis));
			// }
			// '[' => {
			// 	self.context_stack.push(TokenKind::Syntax(SyntaxKind::LeftBracket));
			// 	self.add_token(TokenKind::Syntax(SyntaxKind::LeftBracket));
			// }
			// '{' => {
			// 	self.context_stack.push(TokenKind::Syntax(SyntaxKind::LeftBrace));
			// 	self.add_token(TokenKind::Syntax(SyntaxKind::LeftBrace));
			// }

			// ')' | ']' | '}' => {
			// 	if !self.context_stack.is_empty() {
			// 		self.context_stack.pop();
			// 	}

			// 	let s_kind = match c {
			// 		')' => SyntaxKind::RightParenthesis,
			// 		']' => SyntaxKind::RightBracket,
			// 		_ => SyntaxKind::RightBrace,
			// 	};
			// 	self.add_token(TokenKind::Syntax(s_kind));

			// 	if c == '}' {
			// 		if let Some((quote, is_multiline)) = self.string_stack.pop() {
			// 			self.start = self.current;
			// 			self.start_position = self.position;
			// 			self.continue_string_scan(quote, is_multiline);
			// 		}
			// 	}
			// }

			// '-' => {
			// 	if first == '>' {
			// 		self.handle_operator(c);
			// 	} else if self.is_identifier_start(first) || (first == '$' && second == '{') {
			// 		self.scan_identifier();
			// 	} else {
			// 		self.handle_operator(c);
			// 	}
			// }
			// '0'..='9' => {
			// 	self.scan_number();
			// }

			// 'n' => {
			// 	self.process_unit_suffix("n");
			// }

			// '_' if !first.is_ascii_alphanumeric() && first != '_' => {
			// 	self.handle_operator(c);
			// }

			// _ if self.is_identifier_start(c) => {
			// 	self.scan_identifier();
			// }
			_ => {
				self.add_token(TokenKind::Unknown);
				// self.handle_operator(c);
			}
		}
	}

	#[inline(always)]
	fn as_str(&self) -> &'src str {
		&self.source[self.current..]
	}

	#[inline(always)]
	fn peek(&self, offset: u8) -> char {
		if offset == 0 {
			return self.chars.clone().next().unwrap_or('\0');
		}

		self.chars.clone().nth(offset as usize).unwrap_or('\0')
	}

	#[inline(always)]
	fn first(&self) -> char {
		self.as_str().chars().next().unwrap_or(EOF_CHAR)
	}

	#[inline(always)]
	fn second(&self) -> char {
		let mut chars = self.as_str().chars();
		chars.next();
		chars.next().unwrap_or(EOF_CHAR)
	}

	#[inline(always)]
	fn third(&self) -> char {
		let mut iter = self.chars.clone();
		iter.next();
		iter.next();
		iter.next().unwrap_or(EOF_CHAR)
	}

	#[inline(always)]
	fn advance(&mut self) -> char {
		let c = self.chars.next().unwrap_or(EOF_CHAR);

		if c != EOF_CHAR {
			self.current += c.len_utf8();
			self.position.advance(c);
		}

		#[cfg(debug_assertions)]
		{
			self.prev = c;
		}

		c
	}

	fn add_token_raw(&mut self, t_type: TokenKind) {
		let is_start = if matches!(t_type, TokenKind::Indent(_) | TokenKind::Dedent(_) | TokenKind::Newline) {
			false
		} else {
			let res = self.is_at_line_start;
			if res {
				self.is_at_line_start = false;
			}
			res
		};

		let has_ws = if matches!(t_type, TokenKind::Indent(_) | TokenKind::Dedent(_) | TokenKind::Newline) {
			false
		} else {
			let res = self.had_whitespace;
			self.had_whitespace = false;
			res
		};

		self.tokens.push(Token::new(t_type, is_start, has_ws, None, self.start_position));
	}

	fn add_token(&mut self, t_type: TokenKind) {
		let text = &self.source[self.start..self.current];

		let is_start = self.is_at_line_start;
		if is_start {
			self.is_at_line_start = false;
		}

		let has_ws = self.had_whitespace;
		self.had_whitespace = false;

		self.tokens.push(Token::new(t_type, is_start, has_ws, Some(text), self.start_position));
	}

	fn scan_identifier(&mut self) {
		let bytes = self.as_str().as_bytes();
		let mut idx = 0;

		while idx < bytes.len() {
			let b = bytes[idx];

			if b == b'$' && idx + 1 < bytes.len() && bytes[idx + 1] == b'{' {
				break;
			}

			if b == b'-' {
				if idx + 1 < bytes.len() {
					let next = bytes[idx + 1];
					let is_arrow = next == b'>';
					let is_alnum = next.is_ascii_alphanumeric();
					let is_interpolation = next == b'$' && idx + 2 < bytes.len() && bytes[idx + 2] == b'{';

					if is_alnum && !is_arrow || is_interpolation {
						idx += 1;
						continue;
					} else {
						break;
					}
				} else {
					break;
				}
			}

			if b.is_ascii_alphanumeric() || b == b'_' {
				idx += 1;
			} else if b >= 128 {
				if idx == 0 {
					self.scan_unicode_identifier();
					return;
				}
				for _ in 0..idx {
					self.advance();
				}
				self.scan_unicode_identifier();
				return;
			} else {
				break;
			}
		}

		for _ in 0..idx {
			self.advance();
		}

		let text = &self.source[self.start..self.current];
		let t_type = get_keyword_token(&text).map_or(TokenKind::Identifier, TokenKind::Keyword);

		let is_start = self.is_at_line_start;
		if is_start {
			self.is_at_line_start = false;
		}

		let has_ws = self.had_whitespace;
		self.had_whitespace = false;

		self.tokens.push(Token::new(t_type, is_start, has_ws, Some(text), self.start_position));
	}

	fn scan_unicode_identifier(&mut self) {
		while !self.is_at_end() {
			let c = self.first();

			if c == '$' && self.second() == '{' {
				break;
			}

			if c == '-' {
				let next = self.second();
				if (next.is_alphanumeric() && next != '>') || (next == '$' && self.third() == '{') {
					self.advance();
					continue;
				} else {
					break;
				}
			}

			if c.is_alphanumeric() || c == '_' {
				self.advance();
			} else {
				break;
			}
		}

		let text = &self.source[self.start..self.current];
		let t_type = get_keyword_token(&text).map_or(TokenKind::Identifier, TokenKind::Keyword);

		self.add_token(t_type);
	}

	fn scan_number(&mut self) {
		let mut radix: u32 = 10;
		let bytes = self.as_str().as_bytes();

		#[cfg(debug_assertions)]
		let prev = self.prev;
		#[cfg(not(debug_assertions))]
		let prev = '0';

		if prev == '0' && !bytes.is_empty() {
			let second = bytes[0] as char;
			match second.to_ascii_lowercase() {
				'x' => {
					radix = 16;
					self.advance();
				}
				'b' => {
					radix = 2;
					self.advance();
				}
				'o' => {
					radix = 8;
					self.advance();
				}
				't' => {
					radix = 32;
					self.advance();
				}
				'c' => {
					radix = 33;
					self.advance();
				}
				_ => {}
			}
		}

		self.consume_digits_with_underscore(radix);

		if radix == 10 && self.first() == '.' {
			let next = self.second();
			if next.is_ascii_digit() {
				self.advance();
				self.consume_digits_with_underscore(10);
			}
		}

		let lexeme = &self.source[self.start..self.current];
		self.process_unit_suffix(lexeme);
	}

	fn process_unit_suffix(&mut self, lexeme: &'src str) {
		let initial_current = self.current;
		let initial_position = self.position;
		let is_n_placeholder = lexeme == "n";

		let bytes = self.as_str().as_bytes();
		let mut ws_byte_count = 0;
		while ws_byte_count < bytes.len() {
			let b = bytes[ws_byte_count];
			if b == b' ' || b == b'\t' {
				ws_byte_count += 1;
			} else {
				break;
			}
		}

		let lookahead = &self.as_str()[ws_byte_count..];
		let unit_match_len = self.context.tree.longest_match(lookahead);

		if unit_match_len > 0 {
			let is_valid_boundary = if let Some(nc) = lookahead[unit_match_len..].chars().next() { !(nc.is_alphanumeric() || nc == '_') } else { true };

			if is_valid_boundary {
				let is_start = self.is_at_line_start;
				let has_ws = self.had_whitespace;
				self.had_whitespace = false;
				if is_start {
					self.is_at_line_start = false;
				}

				self.tokens.push(Token::new(TokenKind::Number, is_start, has_ws, Some(lexeme), self.start_position));

				for _ in 0..ws_byte_count {
					self.advance();
				}

				self.start = self.current;
				self.start_position = self.position;

				let unit_lexeme = &lookahead[..unit_match_len];
				let unit_kind = self.context.lookup.get(unit_lexeme).cloned().unwrap_or(UnitKind::None);

				let mut bytes_to_consume = unit_match_len;
				while bytes_to_consume > 0 {
					let c = self.advance();
					bytes_to_consume -= c.len_utf8();
				}

				self.add_token(TokenKind::Unit(unit_kind));
				return;
			}
		}

		if is_n_placeholder {
			self.current = initial_current - 1;
			self.position = Position {
				offset: initial_position.offset - 1,
				line: initial_position.line,
				column: initial_position.column - 1,
			};
			self.chars = self.source[self.current..].chars();
			self.start = self.current;
			self.start_position = self.position;
			self.scan_identifier();
		} else {
			self.add_token(TokenKind::Number);
		}
	}

	fn consume_digits_with_underscore(&mut self, radix: u32) {
		match radix {
			10 => {
				let bytes = self.as_str().as_bytes();
				let mut idx = 0;
				while idx < bytes.len() {
					let b = bytes[idx];
					if b.is_ascii_digit() || b == b'_' {
						idx += 1;
					} else {
						break;
					}
				}
				for _ in 0..idx {
					self.advance();
				}
			}
			16 => {
				let bytes = self.as_str().as_bytes();
				let mut idx = 0;
				while idx < bytes.len() {
					let b = bytes[idx];
					if b.is_ascii_hexdigit() || b == b'_' {
						idx += 1;
					} else {
						break;
					}
				}
				for _ in 0..idx {
					self.advance();
				}
			}
			2 => {
				let bytes = self.as_str().as_bytes();
				let mut idx = 0;
				while idx < bytes.len() {
					let b = bytes[idx];
					if b == b'0' || b == b'1' || b == b'_' {
						idx += 1;
					} else {
						break;
					}
				}
				for _ in 0..idx {
					self.advance();
				}
			}
			8 => {
				let bytes = self.as_str().as_bytes();
				let mut idx = 0;
				while idx < bytes.len() {
					let b = bytes[idx];
					if (b >= b'0' && b <= b'7') || b == b'_' {
						idx += 1;
					} else {
						break;
					}
				}
				for _ in 0..idx {
					self.advance();
				}
			}
			32 => {
				while !self.is_at_end() {
					let c = self.first();
					let lower = c.to_ascii_lowercase();
					if c.is_ascii_digit() || (lower >= 'a' && lower <= 'v') || c == '_' {
						self.advance();
					} else {
						break;
					}
				}
			}
			33 => {
				while !self.is_at_end() {
					let c = self.first();
					let lower = c.to_ascii_lowercase();
					let is_valid = c.is_ascii_digit() || (lower >= 'a' && lower <= 'z' && !"ilou".contains(lower)) || c == '_';
					if is_valid {
						self.advance();
					} else {
						break;
					}
				}
			}
			_ => {
				while !self.is_at_end() {
					let c = self.first();
					if c.is_digit(radix) || c == '_' {
						self.advance();
					} else {
						break;
					}
				}
			}
		}
	}

	fn scan_string(&mut self, quote: char) {
		let is_multiline = quote == '"' && self.first() == '"' && self.second() == '"';

		if is_multiline {
			self.advance();
			self.advance();
		}

		let syntax_kind = match quote {
			'"' => SyntaxKind::DoubleQuote,
			'\'' => SyntaxKind::SingleQuote,
			_ => SyntaxKind::GraveAccent,
		};

		self.add_token(TokenKind::Syntax(syntax_kind));

		self.start = self.current;
		self.start_position = self.position;

		self.continue_string_scan(quote, is_multiline);
	}

	fn continue_string_scan(&mut self, quote: char, is_multiline: bool) {
		let content_start = self.current;

		loop {
			if self.is_at_end() {
				if content_start != self.current {
					self.start = content_start;
					self.add_token(TokenKind::String);
				}
				self.errors_storage.add(Error::span(ErrorKind::Lexical(LexicalError::UnterminatedString), self.start_position, self.position), ErrorFlag::Critical);
				return;
			}

			if self.first() == '$' && self.second() == '{' {
				if content_start != self.current {
					self.start = content_start;
					self.add_token(TokenKind::String);
				}

				self.start = self.current;
				self.start_position = self.position;
				self.advance();
				self.add_token(TokenKind::Syntax(SyntaxKind::Dollar));

				self.start = self.current;
				self.start_position = self.position;
				self.advance();
				self.context_stack.push(TokenKind::Syntax(SyntaxKind::LeftBrace));
				self.add_token(TokenKind::Syntax(SyntaxKind::LeftBrace));

				self.string_stack.push((quote, is_multiline));
				return;
			}

			let is_closing = if is_multiline { self.first() == '"' && self.second() == '"' && self.third() == '"' } else { self.first() == quote };

			if is_closing {
				break;
			}

			if self.first() == '\n' && !is_multiline {
				break;
			}

			let c = self.advance();
			if c == '\\' && !self.is_at_end() {
				self.advance();
			}
		}

		if self.is_at_end() || (self.first() == '\n' && !is_multiline) {
			self.errors_storage.add(Error::span(ErrorKind::Lexical(LexicalError::UnterminatedString), self.start_position, self.position), ErrorFlag::Critical);
			return;
		}

		if content_start != self.current {
			self.start = content_start;
			self.add_token(TokenKind::String);
		}

		self.start = self.current;
		self.start_position = self.position;

		let quote_len = if is_multiline { 3 } else { 1 };
		for _ in 0..quote_len {
			self.advance();
		}

		let syntax_kind = match quote {
			'"' => SyntaxKind::DoubleQuote,
			'\'' => SyntaxKind::SingleQuote,
			_ => SyntaxKind::GraveAccent,
		};

		self.add_token(TokenKind::Syntax(syntax_kind));

		self.start = self.current;
		self.start_position = self.position;
	}

	fn handle_indentation(&mut self) {
		let bytes = self.as_str().as_bytes();
		let mut weight = 0;
		let mut idx = 0;

		while idx < bytes.len() {
			match bytes[idx] {
				b' ' => weight += 1,
				b'\t' => {
					let last_weight = *self.indent_stack.last().unwrap_or(&0);
					weight += if last_weight == 0 { 4 } else { last_weight };
				}
				_ => break,
			}
			idx += 1;
		}

		for _ in 0..idx {
			self.advance();
		}

		if matches!(self.first(), '\n' | '\r') {
			return;
		}

		if self.first() == '/' && ((self.second() == '|' && self.third() == '\\') || self.second() == '*') {
			self.start = self.current;
			self.start_position = self.position;
			return;
		}

		if !self.context_stack.is_empty() {
			self.start = self.current;
			self.start_position = self.position;
			return;
		}

		let last_weight = *self.indent_stack.last().unwrap();

		if weight > last_weight {
			self.indent_stack.push(weight);
			let level = (self.indent_stack.len() - 1) as u8;
			self.add_token_raw(TokenKind::Indent(level));
		} else if weight < last_weight {
			if !self.indent_stack.contains(&weight) {
				self.errors_storage.add(Error::at(ErrorKind::Lexical(LexicalError::InvalidIndentation), self.start_position), ErrorFlag::Critical);
			}

			while weight < *self.indent_stack.last().unwrap() {
				self.indent_stack.pop();
				let level = (self.indent_stack.len() - 1) as u8;
				self.add_token_raw(TokenKind::Dedent(level));
			}
		}

		self.start = self.current;
		self.start_position = self.position;
	}

	fn handle_operator(&mut self, character: char) {
		let first = self.first();
		let second = self.second();
		let operator = match_operator(character, Some(first), Some(second));

		for _ in 0..operator.consume_count {
			self.advance();
		}

		match operator.token_kind {
			TokenKind::LineComment => {
				while !self.is_at_end() {
					let next = self.first();
					if next == '\n' {
						break;
					}
					self.advance();
				}

				self.start = self.current;
				self.start_position = self.position;
			}

			TokenKind::BlockComment => {
				while !self.is_at_end() {
					if self.first() == '*' && self.second() == '/' {
						self.advance();
						self.advance();
						break;
					}
					self.advance();
				}

				self.start = self.current;
				self.start_position = self.position;
			}

			TokenKind::Unknown => {}

			_ => {
				self.add_token(operator.token_kind);
			}
		}
	}

	#[inline(always)]
	fn is_at_end(&self) -> bool {
		self.chars.as_str().is_empty()
	}

	#[inline(always)]
	fn is_identifier_start(&self, character: char) -> bool {
		if character.is_ascii() { character.is_ascii_alphabetic() || character == '_' } else { is_xid_start(character) }
	}
}
