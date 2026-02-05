use lacon_core::shared::unit::{UNITS, UnitKind};

/// Ищет юнит в статическом массиве UNITS и возвращает его формулу
pub fn get_unit_formula(symbol: &str) -> String {
	// Поиск по символу в UNITS
	let unit_def = UNITS.iter().find(|u| u.symbol == symbol);

	if let Some(def) = unit_def {
		format_dimension(def.dimension)
	} else {
		// Если юнит составной или с префиксом, ищем вхождение (простейший вариант)
		format!("Dimension for {}", symbol)
	}
}

/// Превращает Dimension в строку на основе вызова .formula()
fn format_dimension(dim: Dimension) -> String {
	let formula = dim.formula();

	// Если списки num и den пустые, это базовый тип (Length, Time и т.д.)
	if formula.num.is_empty() && formula.den.is_empty() {
		return format!("{:?}", dim);
	}

	let mut res = String::new();

	// Собираем числитель
	for (i, (d, pow)) in formula.num.iter().enumerate() {
		if i > 0 {
			res.push(' ');
		}
		res.push_str(&format!("{:?}^{}", d, pow));
	}

	// Собираем знаменатель
	if !formula.den.is_empty() {
		res.push_str(" / ");
		for (i, (d, pow)) in formula.den.iter().enumerate() {
			if i > 0 {
				res.push(' ');
			}
			res.push_str(&format!("{:?}^{}", d, pow));
		}
	}

	res
}
