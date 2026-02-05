pub use super::super::TokenKind;
pub use super::{OpMatch, OperatorKind};

fn simple(t: TokenKind) -> OpMatch {
	OpMatch { token_kind: t, consume_count: 0 }
}

pub fn match_operator(c1: char, c2: Option<char>, c3: Option<char>) -> OpMatch {
	if !OperatorKind::is_op_char(c1) {
		return match c1 {
			_ => simple(TokenKind::Unknown),
		};
	}

	let mut chars = vec![c1];

	if let Some(next) = c2 {
		if OperatorKind::is_op_char(next) {
			chars.push(next);
			if let Some(next_next) = c3 {
				if OperatorKind::is_op_char(next_next) {
					chars.push(next_next);
				}
			}
		}
	}

	let consume_count = chars.len() - 1;

	let mut current_op: Option<OperatorKind> = None;
	while let Some(ch) = chars.pop() {
		let constructor = OperatorKind::from_char(ch).unwrap();
		current_op = Some(constructor(current_op.map(Box::new)));
	}

	OpMatch {
		token_kind: TokenKind::Operator(current_op.unwrap()),
		consume_count,
	}
}
