use super::token::TokenKind;
use crate::shared::common::characters::*;

/// Структура для результата сопоставления оператора
pub struct OpMatch {
	pub token_type: TokenKind,
	pub consume_count: usize, // Сколько дополнительных символов поглотить (кроме первого)
}

/// Функция сопоставляет символы с соответствующими типами токенов.
pub fn match_operator(c1: char, c2: Option<char>, c3: Option<char>) -> OpMatch {
	match c1 {
		// Односимвольные
		'(' => simple(TokenKind::LeftParen),
		')' => simple(TokenKind::RightParen),
		'{' => simple(TokenKind::LeftBrace),
		'}' => simple(TokenKind::RightBrace),
		'[' => simple(TokenKind::LeftBracket),
		']' => simple(TokenKind::RightBracket),
		',' => simple(TokenKind::Comma),
		';' => simple(TokenKind::Semicolon),

		// Двоеточие и Scope Resolution ::
		':' => match c2 {
			Some(':') => OpMatch {
				token_type: TokenKind::ColonColon,
				consume_count: 1,
			},
			Some('=') => OpMatch {
				token_type: TokenKind::ColonEqual,
				consume_count: 1,
			},
			_ => simple(TokenKind::Colon),
		},

		// Знаки вопроса
		'?' => match c2 {
			Some('?') => OpMatch {
				token_type: TokenKind::QuestionQuestion,
				consume_count: 1,
			},
			_ => simple(TokenKind::Question),
		},

		'^' => match c2 {
			Some('=') => OpMatch {
				token_type: TokenKind::XorEqual,
				consume_count: 1,
			},
			_ => simple(TokenKind::Caret),
		},
		'~' => match c2 {
			Some('=') => OpMatch {
				token_type: TokenKind::TildeEqual,
				consume_count: 1,
			},
			_ => simple(TokenKind::Tilde),
		},
		'@' => match c2 {
			Some('=') => OpMatch {
				token_type: TokenKind::AtEqual,
				consume_count: 1,
			},
			_ => simple(TokenKind::At),
		},
		'#' => match c2 {
			Some('=') => OpMatch {
				token_type: TokenKind::HashEqual,
				consume_count: 1,
			},
			_ => simple(TokenKind::Hash),
		},
		'$' => match c2 {
			Some('{') => OpMatch {
				token_type: TokenKind::DollarLeftBrace,
				consume_count: 1,
			},
			Some('=') => OpMatch {
				token_type: TokenKind::DollarEqual,
				consume_count: 1,
			},
			_ => simple(TokenKind::Dollar),
		},

		// Арифметика и составные операторы
		'+' => match c2 {
			Some('+') => OpMatch {
				token_type: TokenKind::PlusPlus,
				consume_count: 1,
			},
			Some('-') => OpMatch {
				token_type: TokenKind::PlusMinus,
				consume_count: 1,
			},
			Some('=') => OpMatch {
				token_type: TokenKind::PlusEqual,
				consume_count: 1,
			},
			_ => simple(TokenKind::Plus),
		},
		'-' => match c2 {
			Some('-') => OpMatch {
				token_type: TokenKind::MinusMinus,
				consume_count: 1,
			},
			Some('=') => OpMatch {
				token_type: TokenKind::MinusEqual,
				consume_count: 1,
			},
			Some('>') => OpMatch { token_type: TokenKind::Arrow, consume_count: 1 },
			_ => simple(TokenKind::Minus),
		},
		'*' | MULTIPLICATION_SIGN => match c2 {
			Some('*' | MULTIPLICATION_SIGN) => OpMatch {
				token_type: TokenKind::AsteriskAsterisk,
				consume_count: 1,
			},
			Some('=') => OpMatch {
				token_type: TokenKind::AsteriskEqual,
				consume_count: 1,
			},
			_ => simple(TokenKind::Asterisk),
		},
		'/' => match c2 {
			Some('|') if c3 == Some('\\') => OpMatch {
				token_type: TokenKind::LineComment,
				consume_count: 2,
			},
			Some('/') if c3 == Some('=') => OpMatch {
				token_type: TokenKind::SlashSlashEqual,
				consume_count: 2,
			},
			Some('*') => OpMatch {
				token_type: TokenKind::BlockComment,
				consume_count: 1,
			},
			Some('/') => OpMatch {
				token_type: TokenKind::SlashSlash,
				consume_count: 1,
			},
			Some('=') => OpMatch {
				token_type: TokenKind::SlashEqual,
				consume_count: 1,
			},
			_ => simple(TokenKind::Slash),
		},
		'\\' => match c2 {
			Some('\\') => OpMatch {
				token_type: TokenKind::BackslashBackslash,
				consume_count: 1,
			},
			_ => simple(TokenKind::Backslash),
		},
		'%' => match c2 {
			Some('%') => OpMatch {
				token_type: TokenKind::PercentPercent,
				consume_count: 1,
			},
			Some('=') => OpMatch {
				token_type: TokenKind::PercentEqual,
				consume_count: 1,
			},
			_ => simple(TokenKind::Percent),
		},

		// Сравнение и равенство
		'=' => match c2 {
			Some('=') => {
				if let Some('=') = c3 {
					OpMatch {
						token_type: TokenKind::EqualEqualEqual,
						consume_count: 2,
					}
				} else {
					OpMatch {
						token_type: TokenKind::EqualEqual,
						consume_count: 1,
					}
				}
			}
			Some('>') => OpMatch {
				token_type: TokenKind::FatArrow,
				consume_count: 1,
			},
			_ => simple(TokenKind::Equal),
		},
		'!' => match c2 {
			Some('=') => OpMatch {
				token_type: TokenKind::BangEqual,
				consume_count: 1,
			},
			_ => simple(TokenKind::Bang),
		},
		'>' => match c2 {
			Some('>') if c3 == Some('=') => OpMatch {
				token_type: TokenKind::GreaterGreaterEqual,
				consume_count: 2,
			},
			Some('>') => OpMatch {
				token_type: TokenKind::GreaterGreater,
				consume_count: 1,
			},
			Some('=') => OpMatch {
				token_type: TokenKind::GreaterEqual,
				consume_count: 1,
			},
			_ => simple(TokenKind::Greater),
		},
		'<' => match c2 {
			Some('<') if c3 == Some('=') => OpMatch {
				token_type: TokenKind::LessLessEqual,
				consume_count: 2,
			},
			Some('<') => OpMatch {
				token_type: TokenKind::LessLess,
				consume_count: 1,
			},
			Some('=') => OpMatch {
				token_type: TokenKind::LessEqual,
				consume_count: 1,
			},
			Some('|') => OpMatch {
				token_type: TokenKind::PipeBackward,
				consume_count: 1,
			},
			_ => simple(TokenKind::Less),
		},

		// Логика и Пайпы
		'|' => match c2 {
			Some('|') => OpMatch {
				token_type: TokenKind::PipePipe,
				consume_count: 1,
			},
			Some('>') => OpMatch {
				token_type: TokenKind::PipeForward,
				consume_count: 1,
			},
			Some('=') => OpMatch {
				token_type: TokenKind::OrEqual,
				consume_count: 1,
			},
			_ => simple(TokenKind::Pipe),
		},
		'&' => match c2 {
			Some('&') => OpMatch {
				token_type: TokenKind::AmpersandAmpersand,
				consume_count: 1,
			},
			Some('=') => OpMatch {
				token_type: TokenKind::AndEqual,
				consume_count: 1,
			},
			_ => simple(TokenKind::Ampersand),
		},

		// Точка, Эллипсис (...), Диапазон (..) или Append (.=)
		'.' => match c2 {
			Some('.') if c3 == Some('.') => OpMatch {
				token_type: TokenKind::DotDotDot,
				consume_count: 2,
			},
			Some('.') => OpMatch { token_type: TokenKind::DotDot, consume_count: 1 },
			Some('+') => OpMatch {
				token_type: TokenKind::DotPlus,
				consume_count: 1,
			},
			Some('-') => OpMatch {
				token_type: TokenKind::DotMinus,
				consume_count: 1,
			},
			Some('=') => OpMatch {
				token_type: TokenKind::DotEqual,
				consume_count: 1,
			},
			_ => simple(TokenKind::Dot),
		},
		'_' => simple(TokenKind::Underscore),

		// Спец-символы юникода
		// Monus
		DOT_MINUS_SIGN => simple(TokenKind::DotMinus),
		DOT_PLUS_SIGN => simple(TokenKind::DotPlus),
		PLUS_MINUS_SIGN => simple(TokenKind::PlusMinus),
		DIVISION_SIGN => simple(TokenKind::Slash),
		NOT_EQUAL_SIGN => simple(TokenKind::BangEqual),
		LESS_EQUAL_SIGN => simple(TokenKind::LessEqual),
		GREATER_EQUAL_SIGN => simple(TokenKind::GreaterEqual),
		IDENTICAL_TO_SIGN => simple(TokenKind::EqualEqual),
		STRICT_EQUAL_SIGN => simple(TokenKind::EqualEqualEqual),
		SECTION_SIGN => simple(TokenKind::SectionMaker),
		ALMOST_EQUAL_SIGN => simple(TokenKind::TildeEqual),
		ARROW_LEFT_SIGN => simple(TokenKind::LeftArrow),
		ARROW_RIGHT_SIGN => simple(TokenKind::RightArrow),
		ARROW_UP_SIGN => simple(TokenKind::UpArrow),
		ARROW_DOWN_SIGN => simple(TokenKind::DownArrow),
		ARROW_DOUBLE_LEFT_SIGN => simple(TokenKind::LeftDoubleArrow),
		ARROW_DOUBLE_RIGHT_SIGN => simple(TokenKind::RightDoubleArrow),
		ARROW_DOUBLE_UP_SIGN => simple(TokenKind::UpDoubleArrow),
		ARROW_DOUBLE_DOWN_SIGN => simple(TokenKind::DownDoubleArrow),

		_ => simple(TokenKind::Unknown),
	}
}

fn simple(t: TokenKind) -> OpMatch {
	OpMatch { token_type: t, consume_count: 0 }
}
