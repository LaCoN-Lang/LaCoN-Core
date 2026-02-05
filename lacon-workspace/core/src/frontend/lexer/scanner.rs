use super::{SyntaxKind, Token, TokenKind, get_keyword_token, match_operator};
use crate::shared::errors::error::Error;
use crate::shared::errors::error_type::ErrorType;
use crate::shared::errors::error_type::LexicalError;
use crate::shared::position::Position;
use crate::shared::unit::{UNIT_LOOKUP, UNITS_TREE, UnitKind};

use std::path::Path;

pub struct Scanner<'a> {
	source: &'a str,
	tokens: Vec<Token<'a>>,
	start: usize,
	current: usize,
	position: Position,
	start_position: Position,
	indent_stack: Vec<usize>,
	context_stack: Vec<TokenKind>,
	string_stack: Vec<(char, bool)>,
	is_at_line_start: bool,
	had_whitespace: bool,
	pub errors: Vec<Error>,
}

impl<'a> Scanner<'a> {
	pub fn new(source: &'a str) -> Self {
		let start_pos = Position::start();
		Self {
			source,
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
			errors: Vec::new(),
			// aliases: Option<&'a HashMap<String, KeywordKind>>,
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

			';' | ',' | '$' | '@' | '#' | '\\' => {
				let kind = match c {
					';' => SyntaxKind::Semicolon,
					',' => SyntaxKind::Comma,
					'$' => SyntaxKind::Dollar,
					'@' => SyntaxKind::At,
					'#' => SyntaxKind::Hash,
					_ => SyntaxKind::Backslash,
				};
				self.add_token(TokenKind::Syntax(kind));
			}

			'(' | '[' | '{' => {
				let s_kind = match c {
					'(' => SyntaxKind::LeftParenthesis,
					'[' => SyntaxKind::LeftBracket,
					_ => SyntaxKind::LeftBrace,
				};
				self.context_stack.push(TokenKind::Syntax(s_kind));
				self.add_token(TokenKind::Syntax(s_kind));
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
				let next = self.peek();
				let next_next = self.peek_next();
				let is_inf = (next == Some('I') || next == Some('i')) && self.check_infinity(1);

				if next == Some('>') {
					self.handle_operator(c);
				} else if (!is_inf && next.map_or(false, |n| n.is_alphabetic() || n == '_')) || (next == Some('$') && next_next == Some('{')) {
					self.scan_identifier();
				} else {
					self.handle_operator(c);
				}
			}

			_ => {
				if c.is_digit(10) {
					self.scan_number();
				} else if (c == 'I' || c == 'i') && self.check_infinity(0) {
					self.scan_infinity_as_number();
				} else if c == 'n' {
					self.process_unit_suffix("n");
				} else if c == '_' && !self.peek().map_or(false, |next| next.is_alphanumeric() || next == '_') {
					self.handle_operator(c);
				} else if c.is_alphabetic() || c == '_' {
					self.scan_identifier();
				} else {
					self.handle_operator(c);
				}
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

		self.tokens.push(Token::new(t_type, is_start, has_ws, "".into(), None, self.start_position, 0));
	}

	fn add_token(&mut self, t_type: TokenKind) {
		let text = self.get_lexeme();
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
		let text = self.get_lexeme();
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
		while let Some(c) = self.peek() {
			if c == '$' && self.peek_next() == Some('{') {
				break;
			}
			if c == '-' {
				let next = self.peek_next();
				let next_next = self.peek_at(2);
				let is_normal_id_part = next.map_or(false, |n| n.is_alphanumeric()) && next != Some('>');
				let is_link_to_interpolation = next == Some('$') && next_next == Some('{');
				if is_normal_id_part || is_link_to_interpolation {
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
		let text = self.get_lexeme();
		let t_type = get_keyword_token(&text).map_or(TokenKind::Identifier, TokenKind::Keyword);

		self.add_token(t_type);
	}

	fn scan_number(&mut self) {
		let mut radix: u32 = 10;

		let bytes = self.source.as_bytes();

		if bytes[self.start] == b'0' {
			if let Some(second) = self.peek() {
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
		}

		self.consume_digits_with_underscore(radix);

		if radix == 10 && self.peek() == Some('.') {
			if let Some(next) = self.peek_next() {
				if next.is_digit(10) {
					self.advance();
					self.consume_digits_with_underscore(10);
				}
			}
		}

		let value_literal = self.get_slice_str(self.start, self.current);
		self.process_unit_suffix(value_literal);
	}

	fn process_unit_suffix(&mut self, value_literal: &'a str) {
		let initial_current = self.current;
		let initial_position = self.position;
		let is_n_placeholder = value_literal == "n";

		while let Some(b) = self.source.as_bytes().get(self.current) {
			if *b == b' ' || *b == b'\t' {
				self.advance();
			} else {
				break;
			}
		}

		let lookahead = &self.source[self.current..];
		let count = UNITS_TREE.longest_match(lookahead);

		if count > 0 {
			let is_valid_boundary = if let Some(nc) = lookahead[count..].chars().next() { !(nc.is_alphanumeric() && nc != '/') } else { true };

			if is_valid_boundary {
				let unit_lexeme = &lookahead[..count];
				let unit_kind = UNIT_LOOKUP.get(unit_lexeme).cloned().unwrap_or(UnitKind::None);
				let final_value = if is_n_placeholder { "1" } else { value_literal };

				self.add_token_with_literal(TokenKind::Number, final_value);

				self.start = self.current;
				self.start_position = self.position;

				let target_end = self.current + count;
				while self.current < target_end {
					if self.is_at_end() {
						break;
					}
					self.advance();
				}

				self.add_token(TokenKind::Unit(unit_kind));
				return;
			}
		}

		if is_n_placeholder {
			self.current = initial_current - 1;
			self.position = initial_position;
			self.start = self.current;
			self.scan_identifier();
		} else {
			self.current = initial_current;
			self.position = initial_position;
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
		while let Some(c) = self.peek() {
			if c == '_' {
				self.advance();
				continue;
			}
			let is_valid = match radix {
				2 => c == '0' || c == '1',
				8 => c >= '0' && c <= '7',
				10 => c.is_digit(10),
				16 => c.is_digit(16),
				32 => c.is_digit(10) || (c.to_ascii_lowercase() >= 'a' && c.to_ascii_lowercase() <= 'v'),
				33 => {
					let lower = c.to_ascii_lowercase();
					c.is_digit(10) || (lower >= 'a' && lower <= 'z' && !"ilou".contains(lower))
				}
				_ => false,
			};
			if is_valid {
				self.advance();
			} else {
				break;
			}
		}
	}

	fn scan_string(&mut self, quote: char) {
		let is_multiline = quote == '"' && self.match_char('"') && self.match_char('"');

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

		while !self.is_at_end() {
			if self.peek() == Some('$') && self.peek_next() == Some('{') {
				let text = self.get_slice_str(content_start, self.current);
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

			let is_closing = if is_multiline {
				self.peek() == Some('"') && self.peek_next() == Some('"') && self.peek_at(2) == Some('"')
			} else {
				self.peek() == Some(quote)
			};

			if is_closing {
				break;
			}

			if self.peek() == Some('\n') && !is_multiline {
				break;
			}

			let c = self.advance();
			if c == '\\' && !self.is_at_end() {
				self.advance();
			}
		}

		if self.is_at_end() || (self.peek() == Some('\n') && !is_multiline) {
			self.report_error(LexicalError::UnterminatedString);
			return;
		}

		let text = self.get_slice_str(content_start, self.current);
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
		let mut current_weight = 0;

		while let Some(c) = self.peek() {
			match c {
				' ' => {
					current_weight += 1;
					self.advance();
				}
				'\t' => {
					let last_weight = *self.indent_stack.last().unwrap_or(&0);
					current_weight += if last_weight == 0 { 4 } else { last_weight };
					self.advance();
				}
				_ => break,
			}
		}

		if matches!(self.peek(), Some('\n') | Some('\r')) {
			return;
		}
		if self.peek() == Some('/') && (self.peek_next() == Some('|') || self.peek_next() == Some('*')) {
			return;
		}

		if !self.context_stack.is_empty() {
			self.start = self.current;
			self.start_position = self.position;
			return;
		}

		let last_weight = *self.indent_stack.last().unwrap();

		if current_weight > last_weight {
			self.indent_stack.push(current_weight);
			let level = (self.indent_stack.len() - 1) as u8;
			self.add_token_raw(TokenKind::Indent(level));
		} else if current_weight < last_weight {
			if !self.indent_stack.contains(&current_weight) {
				self.report_error(LexicalError::InvalidIndentation);
			}

			while current_weight < *self.indent_stack.last().unwrap() {
				self.indent_stack.pop();
				let level = (self.indent_stack.len() - 1) as u8;
				self.add_token_raw(TokenKind::Dedent(level));
			}
		}

		self.start = self.current;
		self.start_position = self.position;
	}

	fn handle_operator(&mut self, c: char) {
		let op = match_operator(c, self.peek(), self.peek_next());

		for _ in 0..op.consume_count {
			self.advance();
		}

		match op.token_kind {
			TokenKind::LineComment => {
				while let Some(next) = self.peek() {
					if next == '\n' {
						break;
					}
					self.advance();
				}
			}

			TokenKind::BlockComment => {
				while !self.is_at_end() {
					if self.peek() == Some('*') && self.peek_next() == Some('/') {
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

	fn advance(&mut self) -> char {
		let rest = &self.source[self.current..];
		let c = rest.chars().next().expect("Unexpected EOF");
		self.current += c.len_utf8();
		self.position.advance(c);
		c
	}

	fn peek(&self) -> Option<char> {
		self.source[self.current..].chars().next()
	}
	fn peek_next(&self) -> Option<char> {
		let mut iter = self.source[self.current..].chars();
		iter.next(); // Пропускаем текущий
		iter.next() // Берем следующий
	}
	fn peek_at(&self, distance: usize) -> Option<char> {
		self.source[self.current..].chars().nth(distance)
	}
	fn is_at_end(&self) -> bool {
		self.current >= self.source.len()
	}
	fn match_char(&mut self, expected: char) -> bool {
		if self.peek() == Some(expected) {
			self.advance();
			true
		} else {
			false
		}
	}
	fn get_lexeme(&self) -> &'a str {
		&self.source[self.start..self.current]
	}
	fn get_slice_str(&self, start: usize, end: usize) -> &'a str {
		&self.source[start..end]
	}

	fn check_infinity(&self, offset_bytes: usize) -> bool {
		let expected = "nfinity";
		let start_pos = self.current + offset_bytes;

		if start_pos + expected.len() > self.source.len() {
			return false;
		}

		&self.source[start_pos..start_pos + expected.len()] == expected
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
