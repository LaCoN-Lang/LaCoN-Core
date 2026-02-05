use super::{SyntaxKind, Token, TokenKind, get_keyword_token, match_operator};
use crate::shared::errors::error::Error;
use crate::shared::errors::error_type::ErrorType;
use crate::shared::errors::error_type::LexicalError;
use crate::shared::position::Position;
use crate::shared::unit::{UNIT_LOOKUP, UNITS_TREE, UnitKind};

use std::path::Path;
use std::str::Chars;

const EOF_CHAR: char = '\0';

pub struct Scanner<'a> {
	source: &'a str,
	tokens: Vec<Token<'a>>,
	start: usize,
	current: usize,
	chars: Chars<'a>,
	position: Position,
	start_position: Position,
	indent_stack: Vec<usize>,
	context_stack: Vec<TokenKind>,
	string_stack: Vec<(char, bool)>,
	is_at_line_start: bool,
	had_whitespace: bool,
	pub errors: Vec<Error>,
	#[cfg(debug_assertions)]
	prev: char,
}

impl<'a> Scanner<'a> {
	pub fn new(source: &'a str) -> Self {
		let start_pos = Position::start();
		Self {
			source,
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
			errors: Vec::new(),
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

		if self.errors_exist() {
			self.write_errors();
			self.write_errors_to_file(Path::new("lexer_errors.log"));
		}
		&self.tokens
	}

	fn scan_token(&mut self) {
		let c = self.advance();
		let first = self.first();
		let second = self.second();

		match c {
			' ' | '\t' | '\r' => {
				self.had_whitespace = true;
				self.start = self.current;
				self.start_position = self.position;
			}
			'\n' => {
				self.add_token_raw(TokenKind::Newline);
				self.is_at_line_start = true;
				self.had_whitespace = false;
				self.start = self.current;
				self.start_position = self.position;
			}

			'"' | '\'' | '`' => {
				self.scan_string(c);
			}

			';' => self.add_token(TokenKind::Syntax(SyntaxKind::Semicolon)),
			',' => self.add_token(TokenKind::Syntax(SyntaxKind::Comma)),
			'$' => self.add_token(TokenKind::Syntax(SyntaxKind::Dollar)),
			'@' => self.add_token(TokenKind::Syntax(SyntaxKind::At)),
			'#' => self.add_token(TokenKind::Syntax(SyntaxKind::Hash)),
			'\\' => self.add_token(TokenKind::Syntax(SyntaxKind::Backslash)),

			'(' => {
				self.context_stack.push(TokenKind::Syntax(SyntaxKind::LeftParenthesis));
				self.add_token(TokenKind::Syntax(SyntaxKind::LeftParenthesis));
			}
			'[' => {
				self.context_stack.push(TokenKind::Syntax(SyntaxKind::LeftBracket));
				self.add_token(TokenKind::Syntax(SyntaxKind::LeftBracket));
			}
			'{' => {
				self.context_stack.push(TokenKind::Syntax(SyntaxKind::LeftBrace));
				self.add_token(TokenKind::Syntax(SyntaxKind::LeftBrace));
			}

			')' | ']' | '}' => {
				if !self.context_stack.is_empty() {
					self.context_stack.pop();
				}

				let s_kind = match c {
					')' => SyntaxKind::RightParenthesis,
					']' => SyntaxKind::RightBracket,
					_ => SyntaxKind::RightBrace,
				};
				self.add_token(TokenKind::Syntax(s_kind));

				if c == '}' {
					if let Some((quote, is_multiline)) = self.string_stack.pop() {
						self.start = self.current;
						self.start_position = self.position;
						self.continue_string_scan(quote, is_multiline);
					}
				}
			}

			'-' => {
				let is_inf = (first == 'I' || first == 'i') && self.check_infinity(1);

				if first == '>' {
					self.handle_operator(c);
				} else if (!is_inf && (first.is_ascii_alphabetic() || first == '_')) || (first == '$' && second == '{') {
					self.scan_identifier();
				} else {
					self.handle_operator(c);
				}
			}

			'0'..='9' => {
				self.scan_number();
			}

			'I' | 'i' if self.check_infinity(0) => {
				self.scan_infinity_as_number();
			}

			'n' => {
				self.process_unit_suffix("n");
			}

			'_' if !first.is_ascii_alphanumeric() && first != '_' => {
				self.handle_operator(c);
			}

			_ if c.is_ascii_alphabetic() || c == '_' => {
				self.scan_identifier();
			}

			_ if (c as u32) > 127 && c.is_alphabetic() => {
				self.scan_identifier();
			}

			_ => {
				self.handle_operator(c);
			}
		}
	}

	#[inline]
	fn as_str(&self) -> &'a str {
		self.chars.as_str()
	}

	#[inline]
	fn first(&self) -> char {
		self.chars.clone().next().unwrap_or(EOF_CHAR)
	}

	#[inline]
	fn second(&self) -> char {
		let mut iter = self.chars.clone();
		iter.next();
		iter.next().unwrap_or(EOF_CHAR)
	}

	#[inline]
	fn third(&self) -> char {
		let mut iter = self.chars.clone();
		iter.next();
		iter.next();
		iter.next().unwrap_or(EOF_CHAR)
	}

	fn advance(&mut self) -> char {
		let c = self.chars.next().unwrap_or(EOF_CHAR);

		if c != EOF_CHAR {
			self.current += c.len_utf8();
		}

		#[cfg(debug_assertions)]
		{
			self.prev = c;
		}

		self.position.advance(c);
		c
	}

	fn eat_until(&mut self, byte: u8) {
		if let Some(index) = memchr::memchr(byte, self.as_str().as_bytes()) {
			for _ in 0..index {
				self.advance();
			}
		} else {
			while !self.is_at_end() {
				self.advance();
			}
		}
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

		self.tokens.push(Token::new(t_type, is_start, has_ws, "", None, self.start_position, 0));
	}

	fn add_token(&mut self, t_type: TokenKind) {
		let text = &self.source[self.start..self.current];
		let len = text.len();

		let is_start = self.is_at_line_start;
		if is_start {
			self.is_at_line_start = false;
		}

		let has_ws = self.had_whitespace;
		self.had_whitespace = false;

		self.tokens.push(Token::new(t_type, is_start, has_ws, text, None, self.start_position, len));
	}

	fn add_token_with_literal(&mut self, t_type: TokenKind, literal: &'a str) {
		let text = &self.source[self.start..self.current];
		let len = text.len();

		let is_start = self.is_at_line_start;
		if is_start {
			self.is_at_line_start = false;
		}

		let has_ws = self.had_whitespace;
		self.had_whitespace = false;

		self.tokens.push(Token::new(t_type, is_start, has_ws, text, Some(literal), self.start_position, len));
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
		let t_type = get_keyword_token(text).map_or(TokenKind::Identifier, TokenKind::Keyword);

		let is_start = self.is_at_line_start;
		if is_start {
			self.is_at_line_start = false;
		}

		let has_ws = self.had_whitespace;
		self.had_whitespace = false;

		self.tokens.push(Token::new(t_type, is_start, has_ws, text, None, self.start_position, text.len()));
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
		let t_type = get_keyword_token(text).map_or(TokenKind::Identifier, TokenKind::Keyword);

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

		let value_literal = &self.source[self.start..self.current];
		self.process_unit_suffix(value_literal);
	}

	fn process_unit_suffix(&mut self, value_literal: &'a str) {
		let initial_current = self.current;
		let initial_position = self.position;
		let initial_chars = self.chars.clone();
		let is_n_placeholder = value_literal == "n";

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
		let unit_match_len = UNITS_TREE.longest_match(lookahead);

		if unit_match_len > 0 {
			let is_valid_boundary = if let Some(nc) = lookahead[unit_match_len..].chars().next() { !(nc.is_alphanumeric() && nc != '/') } else { true };

			if is_valid_boundary {
				let is_start = self.is_at_line_start;
				let has_ws = self.had_whitespace;
				self.had_whitespace = false;
				if is_start {
					self.is_at_line_start = false;
				}

				let final_value = if is_n_placeholder { "1" } else { value_literal };
				self.tokens.push(Token::new(TokenKind::Number, is_start, has_ws, value_literal, Some(final_value), self.start_position, value_literal.len()));

				for _ in 0..ws_byte_count {
					self.advance();
				}

				self.start = self.current;
				self.start_position = self.position;

				let unit_lexeme = &lookahead[..unit_match_len];
				let unit_kind = UNIT_LOOKUP.get(unit_lexeme).cloned().unwrap_or(UnitKind::None);

				let mut bytes_to_consume = unit_match_len;
				while bytes_to_consume > 0 {
					let c = self.advance();
					bytes_to_consume -= c.len_utf8();
				}

				self.add_token(TokenKind::Unit(unit_kind));
				return;
			}
		}

		self.current = initial_current;
		self.position = initial_position;
		self.chars = initial_chars;

		if is_n_placeholder {
			self.current -= 1;
			self.position.column -= 1;
			self.chars = self.source[self.current..].chars();
			self.start = self.current;
			self.scan_identifier();
		} else {
			self.add_token_with_literal(TokenKind::Number, value_literal);
		}
	}

	fn scan_infinity_as_number(&mut self) {
		for _ in 0..7 {
			self.advance();
		}
		let value_literal: &'static str = "Infinity";
		self.process_unit_suffix(value_literal);
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
				let text = &self.source[content_start..self.current];
				if !text.is_empty() {
					self.add_token_with_literal(TokenKind::String, text);
				}
				self.report_error(LexicalError::UnterminatedString);
				return;
			}

			if self.first() == '$' && self.second() == '{' {
				let text = &self.source[content_start..self.current];
				if !text.is_empty() {
					self.add_token_with_literal(TokenKind::String, text);
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
			self.report_error(LexicalError::UnterminatedString);
			return;
		}

		let text = &self.source[content_start..self.current];
		if !text.is_empty() {
			self.add_token_with_literal(TokenKind::String, text);
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
		if self.first() == '/' && (self.second() == '|' || self.second() == '*') {
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
				self.report_error(LexicalError::InvalidIndentation);
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

	fn handle_operator(&mut self, c: char) {
		let first = self.first();
		let second = self.second();
		let op = match_operator(c, Some(first), Some(second));

		for _ in 0..op.consume_count {
			self.advance();
		}

		match op.token_kind {
			TokenKind::LineComment => {
				self.eat_until(b'\n');
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
			}

			TokenKind::Unknown => {}

			_ => {
				self.add_token(op.token_kind);
			}
		}
	}

	#[inline]
	fn is_at_end(&self) -> bool {
		self.chars.as_str().is_empty()
	}

	fn check_infinity(&self, offset: usize) -> bool {
		let rest = self.as_str();
		let bytes = rest.as_bytes();

		if offset + 7 > bytes.len() {
			return false;
		}

		let expected = b"nfinity";
		&bytes[offset..offset + 7] == expected
	}

	fn report_error(&mut self, error_type: LexicalError) {
		let err = Error::new(ErrorType::Lexical(error_type), Some(self.start_position), Some(self.position));

		self.errors.push(err);
		self.add_token(TokenKind::Error);
	}

	fn write_errors(&self) {
		for error in &self.errors {
			eprintln!("{}", error);
		}
	}

	fn write_errors_to_file(&self, file_path: &Path) {
		use std::fs::OpenOptions;
		use std::io::Write;

		let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(file_path).expect("Не удалось открыть файл для записи ошибок");

		for error in &self.errors {
			writeln!(file, "{}", error).expect("Не удалось записать ошибку в файл");
		}
	}

	fn errors_exist(&self) -> bool {
		!self.errors.is_empty()
	}
}
