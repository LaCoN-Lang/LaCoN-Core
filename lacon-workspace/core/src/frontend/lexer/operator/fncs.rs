use super::super::TokenKind;
pub use super::{OpMatch, OperatorKind};
use crate::shared::common::characters::*;

#[inline(always)]
pub fn match_operator(c1: char, c2: Option<char>, c3: Option<char>) -> OpMatch {
	use OperatorKind::*;
	use TokenKind::{LineComment, Operator};
	let (t_kind, consumed) = match c1 {
		'+' => match c2 {
			Some('+') => (Operator(PlusPlus), 1),
			Some('=') => (Operator(PlusEqual), 1),
			_ => (Operator(Plus), 0),
		},
		'-' | MINUS_SIGN => match c2 {
			Some('-') | Some(MINUS_SIGN) => (Operator(MinusMinus), 1),
			Some('=') => (Operator(MinusEqual), 1),
			Some('>') => (Operator(DashGreater), 1),
			_ => (Operator(Minus), 0),
		},
		'*' => match c2 {
			Some('*') => match c3 {
				Some('=') => (Operator(AsteriskAsteriskEqual), 2),
				_ => (Operator(AsteriskAsterisk), 1),
			},
			Some('=') => (Operator(AsteriskEqual), 1),
			_ => (Operator(Asterisk), 0),
		},
		'/' => match c2 {
			Some('|') if c3 == Some('\\') => (LineComment, 2),
			Some('/') => match c3 {
				Some('=') => (Operator(SlashSlashEqual), 2),
				_ => (Operator(SlashSlash), 1),
			},
			Some('=') => (Operator(SlashEqual), 1),
			_ => (Operator(Slash), 0),
		},
		'=' => match c2 {
			Some('=') => match c3 {
				Some('=') => (Operator(EqualEqualEqual), 2),
				_ => (Operator(EqualEqual), 1),
			},
			Some('>') => (Operator(EqualGreater), 1),
			_ => (Operator(Equal), 0),
		},
		'!' => match c2 {
			Some('=') => match c3 {
				Some('=') => (Operator(NotEqualEqual), 2),
				_ => (Operator(NotEqual), 1),
			},
			Some(':') => (Operator(NotColon), 1),
			_ => (Operator(Exclamation), 0),
		},
		'<' => match c2 {
			Some('-') => match c3 {
				Some('<') => (Operator(LessMinusLess), 2),
				_ => (Operator(LessMinus), 1),
			},
			Some('=') => match c3 {
				Some('=') => (Operator(LessEqualEqual), 2),
				_ => (Operator(LessEqual), 1),
			},
			Some('<') => match c3 {
				Some('-') => (Operator(LessLessDash), 2),
				Some('=') => (Operator(LessLessEqual), 2),
				Some('<') => (Operator(LessLessLess), 2),
				_ => (Operator(LessLess), 1),
			},
			Some('.') => match c3 {
				Some('.') => (Operator(LessDotDot), 2),
				_ => (Operator(Less), 0),
			},
			Some('|') => (Operator(LessPipe), 1),
			_ => (Operator(Less), 0),
		},
		'>' => match c2 {
			Some('=') => (Operator(GreaterEqual), 1),
			Some('>') => match c3 {
				Some('-') => (Operator(GreaterGreaterDash), 2),
				Some('=') => (Operator(GreaterGreaterEqual), 2),
				Some('>') => (Operator(GreaterGreaterGreater), 2),
				_ => (Operator(GreaterGreater), 1),
			},
			Some('-') => match c3 {
				Some('>') => (Operator(GreaterDashGreater), 2),
				_ => (Operator(Greater), 0),
			},
			_ => (Operator(Greater), 0),
		},
		'.' => match c2 {
			Some('.') => match c3 {
				Some('.') => (Operator(DotDotDot), 2),
				Some('=') => (Operator(DotDotEqual), 2),
				Some('<') => (Operator(DotDotLess), 2),
				_ => (Operator(DotDot), 1),
			},
			Some('+') => (Operator(DotPlus), 1),
			Some('-') => (Operator(DotMinus), 1),
			Some('=') => (Operator(DotEqual), 1),
			_ => (Operator(Dot), 0),
		},
		'?' => match c2 {
			Some('?') => (Operator(QuestionQuestion), 1),
			Some('=') => (Operator(QuestionEqual), 1),
			Some(':') => (Operator(QuestionColon), 1),
			Some('.') => (Operator(QuestionDot), 1),
			_ => (Operator(Question), 0),
		},
		'%' => match c2 {
			Some('%') => (Operator(PercentPercent), 1),
			Some('=') => (Operator(PercentEqual), 1),
			_ => (Operator(Percent), 0),
		},
		'^' => match c2 {
			Some('^') => (Operator(CircumflexCircumflex), 1),
			Some('=') => (Operator(CircumflexEqual), 1),
			_ => (Operator(Circumflex), 0),
		},
		'&' => match c2 {
			Some('&') => match c3 {
				Some('=') => (Operator(AmpersandAmpersandEqual), 2),
				_ => (Operator(AmpersandAmpersand), 1),
			},
			Some('=') => (Operator(AmpersandEqual), 1),
			_ => (Operator(Ampersand), 0),
		},
		'|' => match c2 {
			Some('|') => match c3 {
				Some('=') => (Operator(PipePipeEqual), 2),
				_ => (Operator(PipePipe), 1),
			},
			Some('=') => (Operator(PipeEqual), 1),
			Some('>') => (Operator(PipeGreater), 1),
			_ => (Operator(Pipe), 0),
		},
		':' => match c2 {
			Some(':') => (Operator(ColonColon), 1),
			Some('=') => (Operator(ColonEqual), 1),
			_ => (Operator(Colon), 0),
		},
		'~' => match c2 {
			Some('=') => (Operator(TildeEqual), 1),
			_ => (Operator(Tilde), 0),
		},

		// Unicode Single-char Match
		MULTIPLICATION_SIGN => (Operator(Multiplication), 0),
		ELLIPSIS_SIGN => (Operator(Ellipsis), 0),
		DIVISION_SIGN => (Operator(Obelus), 0),
		PLUS_MINUS_SIGN => (Operator(PlusMinus), 0),
		DOT_MINUS_SIGN => (Operator(Monus), 0),
		DOT_PLUS_SIGN => (Operator(DotPlusUni), 0),
		NOT_EQUAL_SIGN => (Operator(NotEqualUni), 0),
		ALMOST_EQUAL_SIGN => (Operator(AlmostEqual), 0),
		IDENTICAL_TO_SIGN => (Operator(IdenticalTo), 0),
		STRICT_EQUAL_SIGN => (Operator(StrictEqualUni), 0),
		LESS_EQUAL_SIGN => (Operator(LessEqualUni), 0),
		GREATER_EQUAL_SIGN => (Operator(GreaterEqualUni), 0),
		XOR_SIGN => (Operator(Xor), 0),
		RING_OPERATOR_SIGN => (Operator(Ring), 0),
		ELEMENT_OF_SIGN => (Operator(ElementOf), 0),
		NOT_AN_ELEMENT_OF_SIGN => (Operator(NotAnElementOf), 0),
		CONTAINS_AS_MEMBER_SIGN => (Operator(ContainsAsMember), 0),
		DOES_NOT_CONTAIN_AS_MEMBER_SIGN => (Operator(DoesNotContainsAsMember), 0),
		FLOOR_START_SIGN => (Operator(FloorStart), 0),
		FLOOR_END_SIGN => (Operator(FloorEnd), 0),
		CEIL_START_SIGN => (Operator(CeilStart), 0),
		CEIL_END_SIGN => (Operator(CeilEnd), 0),

		_ => {
			return OpMatch {
				token_kind: TokenKind::Unknown,
				consume_count: 0,
			};
		}
	};

	OpMatch { token_kind: t_kind, consume_count: consumed }
}
