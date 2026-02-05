macro_rules! unicode_chars {
    ($($name:ident = $char:literal;)*) => {
        paste::paste! {
            $(pub const [<$name _SIGN>]: char = $char;)*
        }
    };
}
unicode_chars! {
				MINUS = '−';
				ASTERISM = '⁂';
				DEGREE = '°';
				PLUS_MINUS = '±';
				MULTIPLICATION = '×';
				DIVISION = '÷';
				DOT_MINUS = '∸';
				DOT_PLUS = '∔';

				LESS_EQUAL = '≤';
				GREATER_EQUAL = '≥';
				NOT_LESS = '≮';
				NOT_GREATER = '≯';

				NOT_EQUAL = '≠';
				ALMOST_EQUAL = '≈';
				IDENTICAL_TO = '≡';
				STRICT_EQUAL = '≣';
				DELTA = 'Δ';
				MICRO = 'μ';
				ANGSTROM = 'Å';
				SECTION = '§';

				FLOOR_START = '⌊';
				FLOOR_END = '⌋';
				CEIL_START = '⌈';
				CEIL_END = '⌉';

				ARROW_LEFT = '←';
				ARROW_RIGHT = '→';
				ARROW_UP = '↑';
				ARROW_DOWN = '↓';
				ARROW_LEFT_RIGHT = '↔';
				ARROW_LEFT_DOWN = '↙';
				ARROW_RIGHT_DOWN = '↘';
				ARROW_LEFT_UP = '↖';
				ARROW_RIGHT_UP = '↗';

				ARROW_DOUBLE_LEFT = '⇐';
				ARROW_DOUBLE_RIGHT = '⇒';
				ARROW_DOUBLE_UP = '⇑';
				ARROW_DOUBLE_DOWN = '⇓';
				ARROW_DOUBLE_LEFT_RIGHT = '⇔';
				ARROW_DOUBLE_LEFT_DOWN = '⇙';
				ARROW_DOUBLE_RIGHT_DOWN = '⇘';
				ARROW_DOUBLE_LEFT_UP = '⇖';
				ARROW_DOUBLE_RIGHT_UP = '⇗';

				SUP_0 = '⁰'; SUP_1 = '¹'; SUP_2 = '²'; SUP_3 = '³';
				SUP_4 = '⁴'; SUP_5 = '⁵'; SUP_6 = '⁶'; SUP_7 = '⁷';
				SUP_8 = '⁸'; SUP_9 = '⁹'; SUP_N = 'ⁿ';

				SUB_0 = '₀'; SUB_1 = '₁'; SUB_2 = '₂'; SUB_3 = '₃';
				SUB_4 = '₄'; SUB_5 = '₅'; SUB_6 = '₆'; SUB_7 = '₇';
				SUB_8 = '₈'; SUB_9 = '₉';
}

#[cfg(test)]
mod testsdd {
	use super::*;

	#[test]
	fn test_unicode_chars() {
		println!("{:04X}", ASTERISM_SIGN as u32);
		println!("{}", DEGREE_SIGN);
		println!("{}", MULTIPLICATION_SIGN);
		println!("{}", DIVISION_SIGN);
		println!("{}", DOT_MINUS_SIGN);
		println!("{}", DOT_PLUS_SIGN);
		println!("{}", NOT_EQUAL_SIGN);
		println!("{}", LESS_EQUAL_SIGN);
		println!("{}", GREATER_EQUAL_SIGN);
		println!("{}", IDENTICAL_TO_SIGN);
		println!("{}", DELTA_SIGN);
		println!("{}", MICRO_SIGN);
		println!("{}", ANGSTROM_SIGN);
	}
}
