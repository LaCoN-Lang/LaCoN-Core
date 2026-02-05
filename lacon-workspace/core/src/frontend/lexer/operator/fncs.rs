pub use super::super::TokenKind;
pub use super::{OpMatch, OperatorKind};

fn simple(t: TokenKind) -> OpMatch {
	OpMatch { token_kind: t, consume_count: 0 }
}

pub fn match_operator(c1: char, c2: Option<char>, c3: Option<char>) -> OpMatch {
	// 1. Проверяем, является ли первый символ оператором вообще
	if !OperatorKind::is_op_char(c1) {
		return match c1 {
			_ => simple(TokenKind::Unknown),
		};
	}

	// 2. Жадное накопление (Maximum Munch)
	// Мы собираем все идущие подряд символы, которые числятся в нашем макросе
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

	// 3. Сколько символов МЫ СЪЕЛИ (помимо первого)
	let consume_count = chars.len() - 1;

	// 4. Сборка рекурсивного дерева (справа налево)
	// Если в chars лежит ['*', '='], то:
	// 1-й шаг: current = Equal(None)
	// 2-й шаг: current = Asterisk(Some(Box(Equal(None))))
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
