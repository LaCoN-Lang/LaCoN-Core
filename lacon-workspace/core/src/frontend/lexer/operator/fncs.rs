use super::super::TokenKind;
pub use super::{OpMatch, OperatorKind};
use crate::shared::characters::*;

#[inline(always)]
pub fn match_operator(tail: &[u8]) -> OpMatch {
	if tail.is_empty() {
		return OpMatch {
			token_kind: TokenKind::Unknown,
			consume_count: 0,
		};
	}
	use OperatorKind::*;
	use TokenKind::{BlockComment, LineComment, Operator};

	let c1 = &tail[0..1];
	let c2 = if tail.len() > 1 { Some(&tail[1..2]) } else { None };
	let c3 = if tail.len() > 2 { Some(&tail[2..3]) } else { None };
	let (t_kind, consumed) = match c1 {
		b"+" => match c2 {
			Some(b"+") => (Operator(PlusPlus), 1),
			Some(b"=") => (Operator(PlusEqual), 1),
			_ => (Operator(Plus), 0),
		},
		b"-" | MINUS_SIGN => match c2 {
			Some(b"-") | Some(MINUS_SIGN) => (Operator(MinusMinus), 1),
			Some(b"=") => (Operator(MinusEqual), 1),
			Some(b">") => (Operator(DashGreater), 1),
			_ => (Operator(Minus), 0),
		},
		b"*" => match c2 {
			Some(b"*") => match c3 {
				Some(b"=") => (Operator(AsteriskAsteriskEqual), 2),
				_ => (Operator(AsteriskAsterisk), 1),
			},
			Some(b"=") => (Operator(AsteriskEqual), 1),
			_ => (Operator(Asterisk), 0),
		},
		b"/" => match c2 {
			Some(b"|') if c3 == Some(b'\\") => (LineComment, 2),
			Some(b"/") => match c3 {
				Some(b"=") => (Operator(SlashSlashEqual), 2),
				_ => (Operator(SlashSlash), 1),
			},
			Some(b"*") => (BlockComment, 1),
			Some(b"=") => (Operator(SlashEqual), 1),
			_ => (Operator(Slash), 0),
		},
		b"=" => match c2 {
			Some(b"=") => match c3 {
				Some(b"=") => (Operator(EqualEqualEqual), 2),
				_ => (Operator(EqualEqual), 1),
			},
			Some(b">") => (Operator(EqualGreater), 1),
			_ => (Operator(Equal), 0),
		},
		b"!" => match c2 {
			Some(b"=") => match c3 {
				Some(b"=") => (Operator(NotEqualEqual), 2),
				_ => (Operator(NotEqual), 1),
			},
			Some(b":") => (Operator(NotColon), 1),
			_ => (Operator(Exclamation), 0),
		},
		b"<" => match c2 {
			Some(b"-") => match c3 {
				Some(b"<") => (Operator(LessMinusLess), 2),
				_ => (Operator(LessMinus), 1),
			},
			Some(b"=") => match c3 {
				Some(b"=") => (Operator(LessEqualEqual), 2),
				_ => (Operator(LessEqual), 1),
			},
			Some(b"<") => match c3 {
				Some(b"-") => (Operator(LessLessDash), 2),
				Some(b"=") => (Operator(LessLessEqual), 2),
				Some(b"<") => (Operator(LessLessLess), 2),
				_ => (Operator(LessLess), 1),
			},
			Some(b".") => match c3 {
				Some(b".") => (Operator(LessDotDot), 2),
				_ => (Operator(Less), 0),
			},
			Some(b"|") => (Operator(LessPipe), 1),
			_ => (Operator(Less), 0),
		},
		b">" => match c2 {
			Some(b"=") => (Operator(GreaterEqual), 1),
			Some(b">") => match c3 {
				Some(b"-") => (Operator(GreaterGreaterDash), 2),
				Some(b"=") => (Operator(GreaterGreaterEqual), 2),
				Some(b">") => (Operator(GreaterGreaterGreater), 2),
				_ => (Operator(GreaterGreater), 1),
			},
			Some(b"-") => match c3 {
				Some(b">") => (Operator(GreaterDashGreater), 2),
				_ => (Operator(Greater), 0),
			},
			_ => (Operator(Greater), 0),
		},
		b"." => match c2 {
			Some(b".") => match c3 {
				Some(b".") => (Operator(DotDotDot), 2),
				Some(b"=") => (Operator(DotDotEqual), 2),
				Some(b"<") => (Operator(DotDotLess), 2),
				_ => (Operator(DotDot), 1),
			},
			Some(b"+") => (Operator(DotPlus), 1),
			Some(b"-") => (Operator(DotMinus), 1),
			Some(b"=") => (Operator(DotEqual), 1),
			_ => (Operator(Dot), 0),
		},
		b"?" => match c2 {
			Some(b"?") => (Operator(QuestionQuestion), 1),
			Some(b"=") => (Operator(QuestionEqual), 1),
			Some(b":") => (Operator(QuestionColon), 1),
			Some(b".") => (Operator(QuestionDot), 1),
			_ => (Operator(Question), 0),
		},
		b"%" => match c2 {
			Some(b"%") => (Operator(PercentPercent), 1),
			Some(b"=") => (Operator(PercentEqual), 1),
			_ => (Operator(Percent), 0),
		},
		b"^" => match c2 {
			Some(b"^") => (Operator(CircumflexCircumflex), 1),
			Some(b"=") => (Operator(CircumflexEqual), 1),
			_ => (Operator(Circumflex), 0),
		},
		b"&" => match c2 {
			Some(b"&") => match c3 {
				Some(b"=") => (Operator(AmpersandAmpersandEqual), 2),
				_ => (Operator(AmpersandAmpersand), 1),
			},
			Some(b"=") => (Operator(AmpersandEqual), 1),
			_ => (Operator(Ampersand), 0),
		},
		b"|" => match c2 {
			Some(b"|") => match c3 {
				Some(b"=") => (Operator(PipePipeEqual), 2),
				_ => (Operator(PipePipe), 1),
			},
			Some(b"=") => (Operator(PipeEqual), 1),
			Some(b">") => (Operator(PipeGreater), 1),
			_ => (Operator(Pipe), 0),
		},
		b":" => match c2 {
			Some(b":") => (Operator(ColonColon), 1),
			Some(b"=") => (Operator(ColonEqual), 1),
			_ => (Operator(Colon), 0),
		},
		b"~" => match c2 {
			Some(b"=") => (Operator(TildeEqual), 1),
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
