use super::{SyntaxKind, Token, TokenKind, get_keyword_token, match_operator};
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
}

impl<'src> Scanner<'src> {
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
		let first = self.peek(0);
		let second = self.second();

		match c {
			b' ' | b'\t' | b'\r' => {
				self.had_whitespace = true;
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

			b'"' | b'\'' | b'`' => {
				self.scan_string(c);
			}

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

			b'-' => {
				if first == b'>' {
					self.handle_operator(c);
				} else if self.is_identifier_start(first) || (first == b'$' && second == b'{') {
					self.scan_identifier();
				} else {
					self.handle_operator(c);
				}
			}

			b'0'..=b'9' => {
				self.scan_number();
			}

			b'n' => {
				self.process_unit_suffix(b"n");
			}

			b'_' if !first.is_ascii_alphanumeric() && first != b'_' => {
				self.handle_operator(c);
			}

			_ if self.is_identifier_start(c) => {
				self.scan_identifier();
			}

			_ => {
				self.handle_operator(c);
			}
		}
	}

	#[inline(always)]
	fn as_slice(&self) -> &'src [u8] {
		&self.source[self.current..]
	}

	#[inline(always)]
	fn peek(&self, offset: u8) -> u8 {
		self.source.get(self.current + offset as usize).copied().unwrap_or(EOF_CHAR)
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
			// Инкрементируем колонку только для стартовых байтов UTF-8
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

		let is_start = self.is_at_line_start;
		if is_start {
			self.is_at_line_start = false;
		}

		let has_ws = self.had_whitespace;
		self.had_whitespace = false;

		self.tokens.push(Token::new(t_type, is_start, has_ws, Some(text), self.start_position));
	}

	fn scan_identifier(&mut self) {
		let bytes = self.as_slice();
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

					if (is_alnum && !is_arrow) || is_interpolation {
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
				// Встретили Unicode символ
				if idx == 0 {
					self.scan_unicode_identifier();
					return;
				}
				// Продвигаем сканер на обработанные ASCII байты
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
		let t_type = get_keyword_token(text).map_or(TokenKind::Identifier, TokenKind::Keyword);

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

			if c == b'$' && self.second() == b'{' {
				break;
			}

			if c == b'-' {
				let next = self.second();
				if (next.is_ascii_alphanumeric() && next != b'>') || (next == b'$' && self.third() == b'{') {
					self.advance();
					continue;
				} else {
					break;
				}
			}

			if c.is_ascii_alphanumeric() || c == b'_' {
				self.advance();
			} else if c >= 128 {
				// Это продолжение Unicode символа, просто продвигаемся
				self.advance();
			} else {
				break;
			}
		}

		let text = &self.source[self.start..self.current];
		let t_type = get_keyword_token(text).map_or(TokenKind::Identifier, TokenKind::Keyword);

		self.add_token(t_type);
	}

	fn scan_number(&mut self) {
		let mut radix: u32 = 10;
		let bytes = self.as_slice();

		#[cfg(debug_assertions)]
		let prev = self.prev;
		#[cfg(not(debug_assertions))]
		let prev = b'0';

		if prev == b'0' && !bytes.is_empty() {
			let second = bytes[0];
			match second.to_ascii_lowercase() {
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

		if radix == 10 && self.first() == b'.' {
			let next = self.second();
			if next.is_ascii_digit() {
				self.advance();
				self.consume_digits_with_underscore(10);
			}
		}

		let lexeme = &self.source[self.start..self.current];
		self.process_unit_suffix(lexeme);
	}

	fn process_unit_suffix(&mut self, lexeme: &'src [u8]) {
		let initial_current = self.current;
		let initial_position = self.position;
		let is_n_placeholder = lexeme == b"n";

		let bytes = self.as_slice();
		let mut ws_byte_count = 0;
		while ws_byte_count < bytes.len() {
			let b = bytes[ws_byte_count];
			if b == b' ' || b == b'\t' {
				ws_byte_count += 1;
			} else {
				break;
			}
		}

		let lookahead = &self.as_slice()[ws_byte_count..];
		let unit_match_len = self.context.tree.longest_match(lookahead);

		if unit_match_len > 0 {
			let is_valid_boundary = if let Some(&nc) = lookahead.get(unit_match_len) { !(nc.is_ascii_alphanumeric() || nc == b'_') } else { true };

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

				// Просто продвигаемся на unit_match_len байт
				for _ in 0..unit_match_len {
					self.advance();
				}

				self.add_token(TokenKind::Unit(unit_kind));
				return;
			}
		}

		if is_n_placeholder {
			// Откатываемся назад на 1 байт ('n' всегда ASCII, = 1 байт)
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

	fn consume_digits_with_underscore(&mut self, radix: u32) {
		match radix {
			10 => {
				let bytes = self.as_slice();
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
				let bytes = self.as_slice();
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
				let bytes = self.as_slice();
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
				let bytes = self.as_slice();
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
					if c.is_ascii_digit() || (lower >= b'a' && lower <= b'v') || c == b'_' {
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
					let is_valid = c.is_ascii_digit() || (lower >= b'a' && lower <= b'z' && !b"ilou".contains(&lower)) || c == b'_';
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
					// Для произвольного radix работаем только с ASCII цифрами
					if (c >= b'0' && c <= b'9') || c == b'_' {
						self.advance();
					} else {
						break;
					}
				}
			}
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
				if content_start != self.current {
					self.start = content_start;
					self.add_token(TokenKind::String);
				}
				self.errors_storage.add(Error::span(ErrorKind::Lexical(LexicalError::UnterminatedString), self.start_position, self.position), ErrorFlag::Critical);
				return;
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

			if is_closing {
				break;
			}

			if self.first() == b'\n' && !is_multiline {
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
		let bytes = self.as_slice();
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

		if matches!(self.first(), b'\n' | b'\r') {
			return;
		}

		if self.first() == b'/' && ((self.second() == b'|' && self.third() == b'\\') || self.second() == b'*') {
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

	fn handle_operator(&mut self, character: u8) {
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
					if next == b'\n' {
						break;
					}
					self.advance();
				}

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

			TokenKind::Unknown => {}

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
		if b < 128 {
			// Быстрый путь через битовую маску для ASCII
			b != 0 && (ASCII_START & (1 << b)) != 0
		} else {
			// Для Unicode просто разрешаем, проверка корректности произойдет позже
			// Это оптимизация - избегаем дорогостоящего декодирования UTF-8
			true
		}
	}

	#[inline(always)]
	fn is_identifier_continue(&self, b: u8) -> bool {
		if b < 128 {
			// Быстрый путь через битовую маску для ASCII
			b != 0 && (ASCII_CONTINUE & (1 << b)) != 0
		} else {
			// Для Unicode просто разрешаем
			true
		}
	}
}
