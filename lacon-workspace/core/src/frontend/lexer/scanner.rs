use super::{KeywordKind, SyntaxKind, Token, TokenKind, match_operator};
use crate::shared::{Error, ErrorFlag, ErrorKind, ErrorStorage, LexicalError, Position, UnitContext, UnitKind};

const ASCII_START: u128 = 0x7fffffe07fffffe0000000000000000;
const ASCII_CONTINUE: u128 = 0x7fffffe87fffffe03ff000000000000;
const EOF_CHAR: u8 = b'\0';

#[derive(Debug)]
pub struct Scanner<'src> {
	source: &'src [u8],
	context: &'src UnitContext<'src>,
	errors_storage: &'src mut ErrorStorage,
	tokens: Vec<Token<'src>>,
	start: usize,
	current: usize,
	position: Position,
	start_position: Position,
	indent_stack: Vec<usize>,
	context_stack: Vec<TokenKind>,
	string_stack: Vec<(u8, bool)>,
	is_at_line_start: bool,
	had_whitespace: bool,
	#[cfg(debug_assertions)]
	prev: u8,
}

impl<'src> Scanner<'src> {
	pub fn reset(&mut self, new_source: &'src [u8]) {
		self.source = new_source;
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

	pub fn new(source: &'src [u8], ctx: &'src UnitContext, errors_storage: &'src mut ErrorStorage) -> Self {
		let start_pos = Position::start();
		Self {
			source,
			context: ctx,
			errors_storage,
			tokens: Vec::new(),
			start: 0,
			current: 0,
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
		self.position.offset += 1;
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

	#[inline(always)]
	fn has_byte(chunk: usize, byte: u8) -> bool {
		let word = chunk ^ (usize::MAX / 255 * byte as usize);
		(word.wrapping_sub(usize::MAX / 255) & !word & (usize::MAX / 255 * 128)) != 0
	}

	#[inline(always)]
	fn fast_skip_line_comment(&mut self) {
		let word_size = std::mem::size_of::<usize>();
		while self.current + word_size <= self.source.len() {
			let chunk = unsafe { (self.source.as_ptr().add(self.current) as *const usize).read_unaligned() };
			if Self::has_byte(chunk, b'\n') {
				break;
			}
			self.current += word_size;
			self.position.offset += word_size;
			self.position.column += word_size;
		}
		let rest = &self.source[self.current..];
		let delta = rest.iter().position(|&b| b == b'\n').unwrap_or(rest.len());

		self.current += delta;
		self.position.offset += delta;
		self.position.column += delta;
	}

	fn scan_token(&mut self) {
		let c = self.advance();
		match c {
			b' ' | b'\t' | b'\r' => {
				self.had_whitespace = true;

				let rest = &self.source[self.current..];
				let delta = rest.iter().position(|&b| !matches!(b, b' ' | b'\t' | b'\r')).unwrap_or(rest.len());

				if delta > 0 {
					self.current += delta;
					self.position.offset += delta;
					self.position.column += delta;
				}

				self.start = self.current;
				self.start_position = self.position;
			}
			b'\n' => {
				self.add_token_raw(TokenKind::Newline);
				self.is_at_line_start = true;
				self.had_whitespace = false;
				self.start = self.current;
				self.start_position = self.position;
			}

			b'"' | b'\'' | b'`' => self.scan_string(c),

			b';' => self.add_token(TokenKind::Syntax(SyntaxKind::Semicolon)),
			b',' => self.add_token(TokenKind::Syntax(SyntaxKind::Comma)),
			b'$' => self.add_token(TokenKind::Syntax(SyntaxKind::Dollar)),
			b'@' => self.add_token(TokenKind::Syntax(SyntaxKind::At)),
			b'#' => self.add_token(TokenKind::Syntax(SyntaxKind::Hash)),
			b'\\' => self.add_token(TokenKind::Syntax(SyntaxKind::Backslash)),

			b'(' => {
				self.context_stack.push(TokenKind::Syntax(SyntaxKind::LeftParenthesis));
				self.add_token(TokenKind::Syntax(SyntaxKind::LeftParenthesis));
			}
			b'[' => {
				self.context_stack.push(TokenKind::Syntax(SyntaxKind::LeftBracket));
				self.add_token(TokenKind::Syntax(SyntaxKind::LeftBracket));
			}
			b'{' => {
				self.context_stack.push(TokenKind::Syntax(SyntaxKind::LeftBrace));
				self.add_token(TokenKind::Syntax(SyntaxKind::LeftBrace));
			}

			b')' | b']' | b'}' => {
				if !self.context_stack.is_empty() {
					self.context_stack.pop();
				}

				let s_kind = match c {
					b')' => SyntaxKind::RightParenthesis,
					b']' => SyntaxKind::RightBracket,
					_ => SyntaxKind::RightBrace,
				};
				self.add_token(TokenKind::Syntax(s_kind));

				if c == b'}' {
					if let Some((quote, is_multiline)) = self.string_stack.pop() {
						self.start = self.current;
						self.start_position = self.position;
						self.continue_string_scan(quote, is_multiline);
					}
				}
			}

			b'0'..=b'9' => self.scan_number(),
			b'n' => self.process_unit_suffix(b"n"),

			_ if self.is_identifier_start(c) => {
				self.scan_identifier();
			}

			_ => self.handle_operator(c),
		}
	}

	#[inline(always)]
	fn first(&self) -> u8 {
		self.source.get(self.current).copied().unwrap_or(EOF_CHAR)
	}

	#[inline(always)]
	fn second(&self) -> u8 {
		self.source.get(self.current + 1).copied().unwrap_or(EOF_CHAR)
	}

	#[inline(always)]
	fn third(&self) -> u8 {
		self.source.get(self.current + 2).copied().unwrap_or(EOF_CHAR)
	}

	#[inline(always)]
	fn advance(&mut self) -> u8 {
		let b = self.first();
		if b == EOF_CHAR {
			return EOF_CHAR;
		}
		self.current += 1;
		self.position.offset += 1;
		if b == b'\n' {
			self.position.line += 1;
			self.position.column = 1;
		} else if (b & 0xC0) != 0x80 {
			self.position.column += 1;
		}
		#[cfg(debug_assertions)]
		{
			self.prev = b;
		}
		b
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
		let is_start = if self.is_at_line_start {
			self.is_at_line_start = false;
			true
		} else {
			false
		};
		let has_ws = self.had_whitespace;
		self.had_whitespace = false;
		self.tokens.push(Token::new(t_type, is_start, has_ws, Some(text), self.start_position));
	}

	fn scan_identifier(&mut self) {
		let source = self.source;
		let total_len = source.len();
		let start_idx = self.current;
		let mut curr_idx = start_idx;
		let mut utf8_check = 0u8;

		if let Some(rest) = source.get(start_idx..) {
			for &b in rest {
				if b < 128 {
					// Основной путь: ASCII символы
					if (ASCII_CONTINUE & (1 << b)) != 0 {
						curr_idx += 1;
						continue;
					}

					if b == b'-' {
						let next_idx = curr_idx + 1;
						if next_idx < total_len {
							let next = unsafe { *source.get_unchecked(next_idx) };
							if next != b'>' && (next >= 128 || (ASCII_CONTINUE & (1 << next)) != 0) {
								curr_idx += 1;
								continue;
							}
						}
					}
					break;
				} else {
					utf8_check |= b;
					curr_idx += 1;
				}
			}
		}

		let text_len = curr_idx - self.start;
		let text = &source[self.start..curr_idx];
		self.current = curr_idx;

		let char_count = if utf8_check < 128 { text.len() } else { text.iter().filter(|&&b| (b & 0xC0) != 0x80).count() };

		self.position.offset = self.start_position.offset + text_len;
		self.position.column = self.start_position.column + char_count;

		let t_type = KeywordKind::from_bytes(text).map(TokenKind::Keyword).unwrap_or(TokenKind::Identifier);

		let is_start = self.is_at_line_start;
		let has_ws = self.had_whitespace;
		self.is_at_line_start = false;
		self.had_whitespace = false;

		self.tokens.push(Token::new(t_type, is_start, has_ws, Some(text), self.start_position));
	}

	fn scan_number(&mut self) {
		let mut radix: u32 = 10;
		#[cfg(debug_assertions)]
		let prev = self.prev;
		#[cfg(not(debug_assertions))]
		let prev = b'0';

		if prev == b'0' {
			match self.first().to_ascii_lowercase() {
				b'x' => {
					radix = 16;
					self.advance();
				}
				b'b' => {
					radix = 2;
					self.advance();
				}
				b'o' => {
					radix = 8;
					self.advance();
				}
				b't' => {
					radix = 32;
					self.advance();
				}
				b'c' => {
					radix = 33;
					self.advance();
				}
				_ => {}
			}
		}

		self.consume_digits_with_underscore(radix);

		if radix == 10 && self.first() == b'.' && self.second().is_ascii_digit() {
			self.advance();
			self.consume_digits_with_underscore(10);
		}

		let lexeme = &self.source[self.start..self.current];
		self.process_unit_suffix(lexeme);
	}

	fn consume_digits_with_underscore(&mut self, radix: u32) {
		let rest = &self.source[self.current..];

		let delta = match radix {
			10 => rest.iter().position(|&b| !(b.is_ascii_digit() || b == b'_')),
			16 => rest.iter().position(|&b| !(b.is_ascii_hexdigit() || b == b'_')),
			_ => None,
		}
		.unwrap_or(rest.len());

		if radix == 10 || radix == 16 {
			self.current += delta;
			self.position.offset += delta;
			self.position.column += delta;
		} else {
			while !self.is_at_end() {
				let b = self.first();
				let is_digit = match radix {
					2 => b == b'0' || b == b'1',
					8 => b >= b'0' && b <= b'7',
					_ => b.is_ascii_digit() || b.to_ascii_lowercase().is_ascii_alphabetic(),
				};
				if is_digit || b == b'_' {
					self.advance();
				} else {
					break;
				}
			}
		}
	}

	fn process_unit_suffix(&mut self, lexeme: &'src [u8]) {
		let initial_current = self.current;
		let initial_position = self.position;
		let lookahead = &self.source[self.current..];

		let mut ws_len = 0;
		while ws_len < lookahead.len() && (lookahead[ws_len] == b' ' || lookahead[ws_len] == b'\t') {
			ws_len += 1;
		}

		let unit_input = &lookahead[ws_len..];
		let unit_match_len = self.context.tree.longest_match(unit_input);

		if unit_match_len > 0 {
			let nc = unit_input.get(unit_match_len).copied().unwrap_or(EOF_CHAR);
			if !(nc.is_ascii_alphanumeric() || nc == b'_') {
				self.add_token(TokenKind::Number);

				self.current += ws_len;
				self.position.offset += ws_len;
				self.position.column += ws_len;

				self.start = self.current;
				self.start_position = self.position;

				let unit_lexeme = &unit_input[..unit_match_len];
				let unit_kind = self.context.lookup.get(unit_lexeme).cloned().unwrap_or(UnitKind::None);

				self.current += unit_match_len;
				self.position.offset += unit_match_len;
				self.position.column += unit_match_len;

				self.add_token(TokenKind::Unit(unit_kind));
				return;
			}
		}

		if lexeme == b"n" {
			self.current = initial_current - 1;
			self.position = Position {
				offset: initial_position.offset - 1,
				line: initial_position.line,
				column: initial_position.column - 1,
			};
			self.start = self.current;
			self.start_position = self.position;
			self.scan_identifier();
		} else {
			self.add_token(TokenKind::Number);
		}
	}

	fn scan_string(&mut self, quote: u8) {
		let is_multiline = quote == b'"' && self.first() == b'"' && self.second() == b'"';
		if is_multiline {
			self.advance();
			self.advance();
		}
		let syntax_kind = match quote {
			b'"' => SyntaxKind::DoubleQuote,
			b'\'' => SyntaxKind::SingleQuote,
			_ => SyntaxKind::GraveAccent,
		};
		self.add_token(TokenKind::Syntax(syntax_kind));
		self.start = self.current;
		self.start_position = self.position;
		self.continue_string_scan(quote, is_multiline);
	}

	fn continue_string_scan(&mut self, quote: u8, is_multiline: bool) {
		let content_start = self.current;
		loop {
			if self.is_at_end() {
				break;
			}

			if self.first() == b'$' && self.second() == b'{' {
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

			let is_closing = if is_multiline { self.first() == b'"' && self.second() == b'"' && self.third() == b'"' } else { self.first() == quote };

			if is_closing || (self.first() == b'\n' && !is_multiline) {
				break;
			}

			let c = self.advance();
			if c == b'\\' && !self.is_at_end() {
				self.advance();
			}
		}

		if self.is_at_end() || (self.first() == b'\n' && !is_multiline) {
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
			b'"' => SyntaxKind::DoubleQuote,
			b'\'' => SyntaxKind::SingleQuote,
			_ => SyntaxKind::GraveAccent,
		};
		self.add_token(TokenKind::Syntax(syntax_kind));
		self.start = self.current;
		self.start_position = self.position;
	}

	fn handle_indentation(&mut self) {
		let mut weight = 0;
		while !self.is_at_end() {
			match self.first() {
				b' ' => {
					weight += 1;
					self.advance();
				}
				b'\t' => {
					let last_weight = *self.indent_stack.last().unwrap_or(&0);
					weight += if last_weight == 0 { 4 } else { last_weight };
					self.advance();
				}
				_ => break,
			}
		}

		if matches!(self.first(), b'\n' | b'\r' | EOF_CHAR) {
			return;
		}
		if self.first() == b'/' && (self.second() == b'|' || self.second() == b'*') {
			return;
		}
		if !self.context_stack.is_empty() {
			return;
		}

		let last_weight = *self.indent_stack.last().unwrap();
		if weight > last_weight {
			self.indent_stack.push(weight);
			self.add_token_raw(TokenKind::Indent((self.indent_stack.len() - 1) as u8));
		} else if weight < last_weight {
			while weight < *self.indent_stack.last().unwrap() {
				self.indent_stack.pop();
				self.add_token_raw(TokenKind::Dedent((self.indent_stack.len() - 1) as u8));
			}
		}
		self.start = self.current;
		self.start_position = self.position;
	}

	fn handle_operator(&mut self, _character: u8) {
		let tail = &self.source[self.current - 1..];
		let operator = match_operator(tail);

		let consume = operator.consume_count;
		if consume > 0 {
			self.current += consume;
			self.position.offset += consume;
			self.position.column += consume;
		}

		match operator.token_kind {
			TokenKind::LineComment => {
				self.fast_skip_line_comment();
				self.start = self.current;
				self.start_position = self.position;
			}
			TokenKind::BlockComment => {
				while !self.is_at_end() {
					if self.first() == b'*' && self.second() == b'/' {
						self.advance();
						self.advance();
						break;
					}
					self.advance();
				}
				self.start = self.current;
				self.start_position = self.position;
			}
			TokenKind::Unknown => {
				self.add_token(TokenKind::Unknown);
			}
			_ => {
				self.add_token(operator.token_kind);
			}
		}
	}

	#[inline(always)]
	fn is_at_end(&self) -> bool {
		self.current >= self.source.len()
	}

	#[inline(always)]
	fn is_identifier_start(&self, b: u8) -> bool {
		if b < 128 { b != 0 && (ASCII_START & (1 << b)) != 0 } else { true }
	}
}
