use super::OperatorKind;

impl OperatorKind {
	pub fn from_str(s: &str) -> Option<Self> {
		let mut chars = s.chars();
		Self::build_recursive(&mut chars)
	}

	fn build_recursive(chars: &mut std::str::Chars) -> Option<Self> {
		let c = chars.next()?;
		let next_op = Self::build_recursive(chars).map(Box::new);

		match c {
			'+' => Some(OperatorKind::Plus(next_op)),
			'-' => Some(OperatorKind::Minus(next_op)),
			'=' => Some(OperatorKind::Equal(next_op)),
			'*' => Some(OperatorKind::Asterisk(next_op)),
			// ... остальные матчи
			_ => None,
		}
	}
}
